use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use opentelemetry::global;
use opentelemetry::propagation::Extractor;
use proto::nova::ratelimit::ratelimiter::HeadersSubmitRequest;
use proto::nova::ratelimit::ratelimiter::{
    ratelimiter_server::Ratelimiter, BucketSubmitTicketRequest,
};
use tokio::sync::RwLock;
use tonic::Response;
use tracing::debug;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use twilight_http_ratelimiting::RatelimitHeaders;

use crate::buckets::bucket::Bucket;
use crate::buckets::redis_lock::RedisLock;
use crate::buckets::GlobalLock;

pub struct RLServer {
    global: Arc<RedisLock>,
    buckets: RwLock<HashMap<String, Arc<Bucket>>>,
}

impl RLServer {
    pub fn new(redis_lock: Arc<RedisLock>) -> Self {
        Self {
            global: redis_lock,
            buckets: RwLock::new(HashMap::new()),
        }
    }
}

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the `MetadataMap`.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the `MetadataMap`.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

#[tonic::async_trait]
impl Ratelimiter for RLServer {
    async fn submit_headers(
        &self,
        request: tonic::Request<HeadersSubmitRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        // Generate a tracing span as usual
        let span = tracing::span!(tracing::Level::INFO, "request process");
        span.set_parent(parent_cx);

        let data = request.into_inner();

        let ratelimit_headers = RatelimitHeaders::from_pairs(
            data.headers.iter().map(|f| (f.0 as &str, f.1.as_bytes())),
        )
        .unwrap();

        if let Some(duration) = self.global.is_locked().await {
            tokio::time::sleep(duration).await;
        }

        let bucket: Arc<Bucket> = if self.buckets.read().await.contains_key(&data.path) {
            self.buckets
                .read()
                .await
                .get(&data.path)
                .expect("impossible")
                .clone()
        } else {
            let bucket = Bucket::new();
            self.buckets.write().await.insert(data.path, bucket.clone());
            bucket
        };

        match ratelimit_headers {
            RatelimitHeaders::Global(global) => {
                // If we are globally ratelimited, we lock using the redis lock
                // This is using redis because a global ratelimit should be executed in all
                // ratelimit workers.
                debug!(
                    "global ratelimit headers detected: {}",
                    global.retry_after()
                );
                self.global
                    .clone()
                    .lock_for(Duration::from_secs(global.retry_after()))
                    .await;
            }
            RatelimitHeaders::None => {}
            RatelimitHeaders::Present(present) => {
                // we should update the bucket.
                bucket.update(&present, data.precise_time);
            }
            _ => unreachable!(),
        };

        Ok(Response::new(()))
    }

    async fn submit_ticket(
        &self,
        request: tonic::Request<BucketSubmitTicketRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        // Generate a tracing span as usual
        let span = tracing::span!(tracing::Level::INFO, "request process");
        span.set_parent(parent_cx);

        let data = request.into_inner();

        let bucket: Arc<Bucket> = if self.buckets.read().await.contains_key(&data.path) {
            self.buckets
                .read()
                .await
                .get(&data.path)
                .expect("impossible")
                .clone()
        } else {
            let bucket = Bucket::new();
            self.buckets.write().await.insert(data.path, bucket.clone());
            bucket
        };

        // wait for the ticket to be accepted
        let _ = bucket.ticket().await;

        Ok(Response::new(()))
    }
}
