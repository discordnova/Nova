use std::{
    sync::{atomic::AtomicU64, Arc},
    time::{Duration, SystemTime},
};

use redis::{aio::MultiplexedConnection, AsyncCommands};
use tokio::sync::Mutex;
use tracing::debug;

/// This is flawed and needs to be replaced sometime with the real RedisLock algorithm
#[derive(Debug)]
pub struct RedisLock {
    redis: Mutex<MultiplexedConnection>,
    is_locked: AtomicU64,
}

impl RedisLock {
    /// Set the global ratelimit as exhausted.
    pub async fn lock_for(self: &Arc<Self>, duration: Duration) {
        debug!("locking globally for {}", duration.as_secs());
        let _: () = self
            .redis
            .lock()
            .await
            .set_ex(
                "nova:rls:lock",
                1,
                (duration.as_secs() + 1).try_into().unwrap(),
            )
            .await
            .unwrap();

        self.is_locked.store(
            (SystemTime::now() + duration)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    pub async fn locked_for(self: &Arc<Self>) -> Option<Duration> {
        let load = self.is_locked.load(std::sync::atomic::Ordering::Relaxed);
        if load != 0 {
            if load
                > SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            {
                return Some(Duration::from_millis(load));
            } else {
                self.is_locked
                    .store(0, std::sync::atomic::Ordering::Relaxed);
            }
        }

        let result = self.redis.lock().await.ttl::<_, i64>("nova:rls:lock").await;
        match result {
            Ok(remaining_time) => {
                if remaining_time > 0 {
                    let duration = Duration::from_secs(remaining_time as u64);
                    debug!("external global lock detected, locking");
                    self.lock_for(duration).await;
                    Some(duration)
                } else {
                    None
                }
            }
            Err(error) => {
                debug!("redis call failed: {}", error);

                None
            }
        }
    }

    pub fn new(redis: MultiplexedConnection) -> Arc<Self> {
        Arc::new(Self {
            redis: Mutex::new(redis),
            is_locked: AtomicU64::new(0),
        })
    }
}
