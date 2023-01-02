use self::bucket::{Bucket, BucketQueueTask};
use shared::redis_crate::{Client, Commands};
use twilight_http_ratelimiting::ticket::{self, TicketNotifier};
use twilight_http_ratelimiting::GetTicketFuture;
mod bucket;

use futures_util::future;
use std::{
    collections::hash_map::{Entry, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug)]
struct RedisLockPair(tokio::sync::Mutex<Client>);

impl RedisLockPair {
    /// Set the global ratelimit as exhausted.
    pub async fn lock_for(&self, duration: Duration) {
        let _: () = self
            .0
            .lock()
            .await
            .set_ex(
                "nova:rls:lock",
                1,
                (duration.as_secs() + 1).try_into().unwrap(),
            )
            .unwrap();
    }

    pub async fn is_locked(&self) -> bool {
        self.0.lock().await.exists("nova:rls:lock").unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RedisGlobalLocalBucketRatelimiter {
    buckets: Arc<Mutex<HashMap<String, Arc<Bucket>>>>,

    global: Arc<RedisLockPair>,
}

impl RedisGlobalLocalBucketRatelimiter {
    #[must_use]
    pub fn new(redis: tokio::sync::Mutex<Client>) -> Self {
        Self {
            buckets: Arc::default(),
            global: Arc::new(RedisLockPair(redis)),
        }
    }

    fn entry(&self, path: String, tx: TicketNotifier) -> Option<Arc<Bucket>> {
        let mut buckets = self.buckets.lock().expect("buckets poisoned");

        match buckets.entry(path.clone()) {
            Entry::Occupied(bucket) => {
                tracing::debug!("got existing bucket: {path:?}");

                bucket.get().queue.push(tx);

                tracing::debug!("added request into bucket queue: {path:?}");

                None
            }
            Entry::Vacant(entry) => {
                tracing::debug!("making new bucket for path: {path:?}");

                let bucket = Bucket::new(path);
                bucket.queue.push(tx);

                let bucket = Arc::new(bucket);
                entry.insert(Arc::clone(&bucket));

                Some(bucket)
            }
        }
    }

    pub fn ticket(&self, path: String) -> GetTicketFuture {
        tracing::debug!("getting bucket for path: {path:?}");

        let (tx, rx) = ticket::channel();

        if let Some(bucket) = self.entry(path.clone(), tx) {
            tokio::spawn(
                BucketQueueTask::new(
                    bucket,
                    Arc::clone(&self.buckets),
                    Arc::clone(&self.global),
                    path,
                )
                .run(),
            );
        }

        Box::pin(future::ok(rx))
    }
}
