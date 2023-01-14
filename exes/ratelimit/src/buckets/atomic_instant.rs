use std::{
    hash::Hash,
    ops::{Add, AddAssign, Sub},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Instant implementation based on an atomic number
/// # Example
/// ```
/// # use ratelimit::buckets::atomic_instant::AtomicInstant;
/// # use std::time::Duration;
///
/// let now = AtomicInstant::now();
/// let max_seconds = u64::MAX / 1_000_000_000;
/// let duration = Duration::new(max_seconds, 0);
/// println!("{:?}", now + duration);
/// ```
#[derive(Default, Debug)]
#[cfg(not(target_feature = "atomic128"))]
pub struct AtomicInstant(AtomicU64);

impl AtomicInstant {
    /// Calculates the duration since the instant.
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::now();
    /// std::thread::sleep(Duration::from_secs(1));
    /// 
    /// assert_eq!(instant.elapsed().as_secs(), 1);
    /// ```
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
    /// Gets the current time in millis
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// instant.set_millis(1000);
    /// 
    /// assert_eq!(instant.as_millis(), 1000);
    /// ```
    pub fn as_millis(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    /// Creates an instant at the current time
    /// # Safety
    /// Truncates if the current unix time is greater than `u64::MAX`
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn now() -> Self {
        Self(AtomicU64::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_millis() as u64,
        ))
    }

    /// Sets the unix time of the instant
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// instant.set_millis(1000);
    /// 
    /// assert_eq!(instant.as_millis(), 1000);
    /// ```
    pub fn set_millis(&self, millis: u64) {
        self.0.store(millis, Ordering::Relaxed);
    }

    /// Determines if the current instant is at the default value
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// 
    /// assert!(instant.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.as_millis() == 0
    }
}

impl Add<Duration> for AtomicInstant {
    type Output = Self;
    /// # Safety
    /// This panics if the right hand side is greater than `i64::MAX`
    /// You can remedy to this using the 128bits feature with changes the
    /// underlying atomic.
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    ///
    /// // we add one second to our instant
    /// instant = instant + Duration::from_secs(1);
    ///
    /// // should be equal to a second
    /// assert_eq!(instant.as_millis(), 1000);
    /// ```
    fn add(self, rhs: Duration) -> Self::Output {
        self.0
            .fetch_add(rhs.as_millis().try_into().unwrap(), Ordering::Relaxed);
        self
    }
}

impl AddAssign<Duration> for AtomicInstant {
    /// # Safety
    /// This panics if the right hand side is greater than `i64::MAX`
    /// You can remedy to this using the 128bits feature with changes the
    /// underlying atomic.
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    ///
    /// // we add one second to our instant
    /// instant += Duration::from_secs(1);
    ///
    /// // should be equal to a second
    /// assert_eq!(instant.as_millis(), 1000);
    /// ```
    fn add_assign(&mut self, rhs: Duration) {
        self.0
            .fetch_add(rhs.as_millis().try_into().unwrap(), Ordering::Relaxed);
    }
}

impl Hash for AtomicInstant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.load(Ordering::Relaxed).hash(state);
    }
}

impl PartialEq for AtomicInstant {
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// let mut instant2 = AtomicInstant::default();
    ///
    /// assert_eq!(instant, instant2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.0.load(Ordering::Relaxed) == other.0.load(Ordering::Relaxed)
    }
}
impl Eq for AtomicInstant {}

impl PartialOrd for AtomicInstant {
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// let mut instant2 = AtomicInstant::default();
    ///
    /// assert!(instant == instant2);
    /// instant.set_millis(1000);
    /// assert!(instant > instant2);
    /// instant.set_millis(0);
    /// instant2.set_millis(1000);
    /// assert!(instant < instant2);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0
            .load(Ordering::Relaxed)
            .partial_cmp(&other.0.load(Ordering::Relaxed))
    }
}

impl Ord for AtomicInstant {
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// let mut instant2 = AtomicInstant::default();
    ///
    /// assert!(instant == instant2);
    /// instant.set_millis(1000);
    /// assert!(instant > instant2);
    /// instant.set_millis(0);
    /// instant2.set_millis(1000);
    /// assert!(instant < instant2);
    /// ```
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .load(Ordering::Relaxed)
            .cmp(&other.0.load(Ordering::Relaxed))
    }
}

impl Sub<Duration> for AtomicInstant {
    type Output = Self;
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// instant.set_millis(1000);
    ///
    /// instant = instant - Duration::from_secs(1);
    ///
    /// assert!(instant.is_empty());
    /// ```
    fn sub(self, rhs: Duration) -> Self::Output {
        self.0
            .fetch_sub(rhs.as_millis().try_into().unwrap(), Ordering::Relaxed);
        self
    }
}

impl Sub<Self> for AtomicInstant {
    type Output = Self;
    /// # Example
    /// ```
    /// # use ratelimit::buckets::atomic_instant::AtomicInstant;
    /// # use std::time::Duration;
    /// let mut instant = AtomicInstant::default();
    /// let mut instant2 = AtomicInstant::default();
    /// instant.set_millis(1000);
    /// instant2.set_millis(2000);
    ///
    /// instant = instant2 - instant;
    ///
    /// assert_eq!(instant.as_millis(), 1000);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        self.0
            .fetch_sub(rhs.0.load(Ordering::Relaxed), Ordering::Relaxed);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicInstant;

    #[test]
    fn should_detect_default() {
        let instant = AtomicInstant::default();
        assert!(instant.is_empty());

        instant.set_millis(1000);
        assert!(!instant.is_empty());
    }
}
