use common::redis_crate::{AsyncCommands, RedisError, aio::Connection};
use hyper::{Body, Request};
use tokio::sync::Mutex;
use std::sync::Arc;
use xxhash_rust::xxh32::xxh32;

pub struct Ratelimiter {
    redis: Arc<Mutex<Connection>>
}

impl Ratelimiter {
    pub fn new(redis: Arc<Mutex<Connection>>) -> Ratelimiter {
        return Ratelimiter {
            redis
        }
    }

    pub async fn check(&mut self,request: Request<Body>) -> bool {
        // we lookup if the route hash is stored in the redis table
        let path = request.uri().path();
        let hash = xxh32(path.as_bytes(), 32);
        let key = format!("nova:rest:ratelimit:url_store:{}", hash);
        let mut redis = self.redis.lock().await;
        let value: Result<String, RedisError> = redis.get(key).await;

        match value {
            Ok(_) => true,
            Err(error) => false,
        }
    }
}
