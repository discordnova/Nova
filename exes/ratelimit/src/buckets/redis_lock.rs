use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc},
    time::{Duration, SystemTime},
};

use redis::{aio::MultiplexedConnection, AsyncCommands};
use tokio::sync::Mutex;
use tracing::debug;

use super::GlobalLock;

/// This is flawed and needs to be replaced sometime with the real `RedisLock` algorithm
#[derive(Debug)]
pub struct RedisLock {
    redis: Mutex<MultiplexedConnection>,
    is_locked: AtomicU64,
}

impl RedisLock {
    #[must_use]
    pub fn new(redis: MultiplexedConnection) -> Arc<Self> {
        Arc::new(Self {
            redis: Mutex::new(redis),
            is_locked: AtomicU64::new(0),
        })
    }
}

impl GlobalLock for RedisLock {
    fn lock_for<'a>(
        self: &'a Arc<Self>,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
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

            // Integer truncating is expected
            #[allow(clippy::cast_possible_truncation)]
            self.is_locked.store(
                (SystemTime::now() + duration)
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        })
    }

    fn is_locked<'a>(
        self: &'a Arc<Self>,
    ) -> Pin<Box<dyn Future<Output = Option<Duration>> + Send + 'a>> {
        Box::pin(async move {
            let load = self.is_locked.load(std::sync::atomic::Ordering::Relaxed);
            if load != 0 {
                // Integer truncating is expected
                #[allow(clippy::cast_possible_truncation)]
                if load
                    > SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64
                {
                    return Some(Duration::from_millis(load));
                }
                self.is_locked
                    .store(0, std::sync::atomic::Ordering::Relaxed);
            }

            let result = self.redis.lock().await.ttl::<_, i64>("nova:rls:lock").await;
            match result {
                Ok(remaining_time) => {
                    if remaining_time > 0 {
                        // Sign loss is allowed since we know it's a positive number
                        // because a ttl is always positive when the key exists and have a ttl
                        // otherwise redis *will* return a negative number, hence the check for
                        // a positive sign.
                        #[allow(clippy::cast_sign_loss)]
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
        })
    }
}
