use shared::{
    log::debug,
    redis_crate::{aio::Connection, AsyncCommands}, error::GenericError,
};
use hyper::{Body, Request, Response};
use std::{
    convert::TryInto,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;
use xxhash_rust::xxh32::xxh32;

pub enum RatelimiterResponse {
    NoSuchUrl,
    Ratelimited,
    Pass,
}

pub struct Ratelimiter {
    redis: Arc<Mutex<Connection>>,
}

impl Ratelimiter {
    pub fn new(redis: Arc<Mutex<Connection>>) -> Ratelimiter {
        return Ratelimiter { redis };
    }

    pub async fn before_request(
        &self,
        request: &Request<Body>,
    ) -> Result<RatelimiterResponse, GenericError> {
        // we lookup if the route hash is stored in the redis table
        let path = request.uri().path();
        let hash = xxh32(path.as_bytes(), 32);
        let mut redis = self.redis.lock().await;

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // global rate litmit
        match redis
            .get::<String, Option<i32>>(format!(
                "nova:rest:ratelimit:global:{}",
                since_the_epoch.as_secs()
            ))
            .await
        {
            Ok(value) => {
                match value {
                    Some(value) => {
                        debug!("incr: {}", value);
                        if value >= 49 {
                            return Ok(RatelimiterResponse::Ratelimited);
                        }
                    }
                    None => {
                        let key =
                            format!("nova:rest:ratelimit:global:{}", since_the_epoch.as_secs());
                        // init global ratelimit
                        redis.set_ex::<String, i32, ()>(key, 0, 2).await.unwrap();
                    }
                }
            }
            Err(_) => {
                return Err(GenericError::StepFailed("radis ratelimit check".to_string()));
            }
        };

        // we lookup the corresponding bucket for this url
        match redis
            .get::<String, Option<String>>(format!("nova:rest:ratelimit:url_bucket:{}", hash))
            .await
        {
            Ok(bucket) => match bucket {
                Some(bucket) => {
                    match redis
                        .exists::<String, bool>(format!("nova:rest:ratelimit:lock:{}", bucket))
                        .await
                    {
                        Ok(exists) => {
                            if exists {
                                Ok(RatelimiterResponse::Ratelimited)
                            } else {
                                Ok(RatelimiterResponse::Pass)
                            }
                        }
                        Err(_) =>  Err(GenericError::StepFailed("radis ratelimit check".to_string())),
                    }
                }
                None => Ok(RatelimiterResponse::NoSuchUrl),
            },
            Err(_) => Err(GenericError::StepFailed("radis ratelimit check".to_string())),
        }
    }

    fn parse_headers(&self, response: &Response<Body>) -> Option<(String, i32, i32)> {
        if let Some(bucket) = response.headers().get("X-RateLimit-Bucket") {
            let bucket = bucket.to_str().unwrap().to_string();

            let remaining = response.headers().get("X-RateLimit-Remaining").unwrap();
            let reset = response.headers().get("X-RateLimit-Reset-After").unwrap();

            let remaining_i32 = remaining.to_str().unwrap().parse::<i32>().unwrap();
            let reset_ms_i32 = reset.to_str().unwrap().parse::<f32>().unwrap().ceil() as i32;
            return Some((bucket, remaining_i32, reset_ms_i32));
        } else {
            None
        }
    }

    pub async fn after_request(&self, path: &str, response: &Response<Body>) {
        let hash = xxh32(path.as_bytes(), 32);
        // verified earlier

        let mut redis = self.redis.lock().await;

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        redis
            .incr::<String, i32, ()>(
                format!("nova:rest:ratelimit:global:{}", since_the_epoch.as_secs()),
                1,
            )
            .await
            .unwrap();
        if let Some((bucket, remaining, reset)) = self.parse_headers(response) {
            if remaining <= 1 {
                // we set a lock for the bucket until the timeout passes
                redis
                    .set_ex::<String, bool, ()>(
                        format!("nova:rest:ratelimit:lock:{}", bucket),
                        true,
                        reset.try_into().unwrap(),
                    )
                    .await
                    .unwrap();
            }

            redis
                .set_ex::<String, String, ()>(
                    format!("nova:rest:ratelimit:url_bucket:{}", hash),
                    bucket,
                    reset.try_into().unwrap(),
                )
                .await
                .unwrap();
        }
    }
}
