use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tracing::debug;

#[derive(Default, Debug)]
pub struct AtomicInstant(AtomicU64);

impl AtomicInstant {
    #[must_use]
    pub const fn empty() -> Self {
        Self(AtomicU64::new(0))
    }

    pub fn elapsed(&self) -> Duration {
        // Truncation is expected
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_millis(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_millis() as u64
                - self.0.load(Ordering::Relaxed),
        )
    }

    pub fn as_millis(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    pub fn set_millis(&self, millis: u64) {
        // get address of struct
        let b = self as *const _ as usize;
        debug!(millis, this = ?b, "settings instant millis");
        self.0.store(millis, Ordering::Relaxed);
    }

    pub fn is_empty(&self) -> bool {
        let millis = self.as_millis();
        // get address of struct
        let b = self as *const _ as usize;
        debug!(millis, this = ?b, "settings instant millis");
        debug!(empty = (millis == 0), millis, this = ?b, "instant empty check");
        millis == 0
    }
}
