use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Default, Debug)]
pub struct AtomicInstant(AtomicU64);

impl AtomicInstant {
    pub const fn empty() -> Self {
        Self(AtomicU64::new(0))
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64 as u64
                - self.0.load(Ordering::SeqCst),
        )
    }

    pub fn as_millis(&self) -> u64 {
        self.0.load(Ordering::SeqCst)
    }

    pub fn set_millis(&self, millis: u64) {
        self.0.store(millis, Ordering::SeqCst);
    }

    pub fn is_empty(&self) -> bool {
        self.as_millis() == 0
    }
}
