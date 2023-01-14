use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

pub mod async_queue;
pub mod atomic_instant;
pub mod bucket;
pub mod noop_lock;
pub mod redis_lock;

pub trait GlobalLock: Send + Sync {
    fn lock_for<'a>(
        self: &'a Arc<Self>,
        duration: Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn is_locked<'a>(
        self: &'a Arc<Self>,
    ) -> Pin<Box<dyn Future<Output = Option<Duration>> + Send + 'a>>;
}
