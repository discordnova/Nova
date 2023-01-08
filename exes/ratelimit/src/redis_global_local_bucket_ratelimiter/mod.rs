use self::bucket::{Bucket, BucketQueueTask};
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use tokio::sync::Mutex;
use twilight_http_ratelimiting::ticket::{self, TicketNotifier};
use twilight_http_ratelimiting::GetTicketFuture;
mod bucket;
use std::future;
use std::{
    collections::hash_map::{Entry, HashMap},
    sync::Arc,
    time::Duration,
};

#[derive(Debug)]
struct RedisLockPair(Mutex<MultiplexedConnection>);

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
            .await
            .unwrap();
    }

    pub async fn is_locked(&self) -> bool {
        self.0.lock().await.exists("nova:rls:lock").await.unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RedisGlobalLocalBucketRatelimiter {
    buckets: Arc<std::sync::Mutex<HashMap<String, Arc<Bucket>>>>,

    global: Arc<RedisLockPair>,
}

impl RedisGlobalLocalBucketRatelimiter {
    #[must_use]
    pub fn new(redis: MultiplexedConnection) -> Self {
        Self {
            buckets: Arc::default(),
            global: Arc::new(RedisLockPair(Mutex::new(redis))),
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

        Box::pin(future::ready(Ok(rx)))
    }
}
