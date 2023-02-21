use super::{async_queue::AsyncQueue, atomic_instant::AtomicInstant};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};
use tracing::{debug, trace};
use twilight_http_ratelimiting::headers::Present;

#[derive(Clone, Debug)]
pub enum TimeRemaining {
    Finished,
    NotStarted,
    Some(Duration),
}

/// A bucket is a simple atomic implementation of a bucket used for ratelimiting
/// It can be updated dynamically depending on the discord api responses.
///
/// # Usage
/// ```
/// # use ratelimit::buckets::bucket::Bucket;
/// # use twilight_http_ratelimiting::RatelimitHeaders;
/// # use std::time::SystemTime;
/// # tokio_test::block_on(async {
///
///     let bucket = Bucket::new();
///
///     // Feed the headers informations into the bucket to update it
///     let headers = [
///         ( "x-ratelimit-bucket", "bucket id".as_bytes()),
///         ("x-ratelimit-limit", "100".as_bytes()),
///         ("x-ratelimit-remaining", "0".as_bytes()),
///         ("x-ratelimit-reset", "99999999999999".as_bytes()),
///         ("x-ratelimit-reset-after", "10.000".as_bytes()),
///     ];
///
///     // Parse the headers
///     let present = if let Ok(RatelimitHeaders::Present(present))
///         = RatelimitHeaders::from_pairs(headers.into_iter()) {
///         present
///     } else { todo!() };
///
///     // this should idealy the time of the request
///     let current_time = SystemTime::now()
///         .duration_since(SystemTime::UNIX_EPOCH)
///         .unwrap()
///         .as_millis() as u64;
///
///     bucket.update(&present, current_time);
/// # })
/// ```
///
/// # Async
/// You need to call this struct new method in a tokio 1.x async runtime.
#[derive(Debug)]
pub struct Bucket {
    /// Limits of tickets that can be accepted
    pub limit: AtomicU64,
    /// Remaining requests that can be executed
    pub remaining: AtomicU64,
    /// Time to wait after [`Self::last_update`] before accepting new tickets.
    pub reset_after: AtomicU64,
    /// Last update got from the discord upstream
    pub last_update: AtomicInstant,

    /// List of tasks that dequeue tasks from [`Self::queue`]
    tasks: Vec<JoinHandle<()>>,
    /// Queue of tickets to be processed.
    queue: AsyncQueue<Sender<()>>,
}

impl Drop for Bucket {
    /// Simply abord the dequeue tasks to aboid leaking memory via arc(s)
    fn drop(&mut self) {
        for join in &self.tasks {
            join.abort();
        }
    }
}

impl Bucket {
    /// Creates a new bucket with four dequeue tasks
    /// # Async
    /// This functions **should** be called in a tokio 1.x runtime, otherwise the function *will* panic.
    #[must_use]
    pub fn new() -> Arc<Self> {
        let tasks = vec![];

        let this = Arc::new(Self {
            limit: AtomicU64::new(u64::max_value()),
            queue: AsyncQueue::default(),
            remaining: AtomicU64::new(u64::max_value()),
            reset_after: AtomicU64::new(u64::max_value()),
            last_update: AtomicInstant::default(),
            tasks,
        });

        // Run with 4 dequeue tasks
        for _ in 0..4 {
            let this = this.clone();
            tokio::spawn(async move {
                // continuously wait for elements in the queue to process them sequantially.
                // this is using parallel tasks to allow (hopefully) better performance.
                while let Some(element) = this.queue.pop().await {
                    if this.remaining() == 0 {
                        debug!("0 tickets remaining, we have to wait.");

                        match this.time_remaining() {
                            TimeRemaining::Finished => {
                                debug!("waiting seems finished.");
                                this.try_reset();
                            }
                            TimeRemaining::Some(duration) => {
                                debug!(milliseconds=%duration.as_millis(), "waiting for ratelimit");
                                tokio::time::sleep(duration).await;

                                this.try_reset();
                            }
                            TimeRemaining::NotStarted => {
                                debug!("we should not wait");
                            }
                        }
                    }

                    this.remaining.fetch_sub(1, Ordering::Relaxed);
                    let _ = element
                        .send(())
                        .map_err(|_| trace!("response channel was closed."));
                }
            });
        }

        this
    }

    /// Total number of tickets allowed in a cycle.
    pub fn limit(&self) -> u64 {
        self.limit.load(Ordering::Relaxed)
    }

    /// Number of tickets remaining in the current cycle.
    pub fn remaining(&self) -> u64 {
        self.remaining.load(Ordering::Relaxed)
    }

    /// Duration after the [`Self::last_update`] time the bucket will refresh.
    pub fn reset_after(&self) -> u64 {
        self.reset_after.load(Ordering::Relaxed)
    }

    /// Time remaining until this bucket will reset.
    pub fn time_remaining(&self) -> TimeRemaining {
        let reset_after = self.reset_after();
        let last_update = &self.last_update;

        if last_update.is_empty() {
            debug!("last update is empty");

            TimeRemaining::NotStarted
        } else {
            let elapsed = last_update.elapsed();

            if elapsed > Duration::from_millis(reset_after) {
                return TimeRemaining::Finished;
            }

            TimeRemaining::Some(Duration::from_millis(reset_after) - elapsed)
        }
    }

    /// Try to reset this bucket's [`Self::last_update`] value if it has finished.
    ///
    /// Returns whether resetting was possible.
    pub fn try_reset(&self) -> bool {
        if self.last_update.is_empty() {
            return false;
        }

        if matches!(self.time_remaining(), TimeRemaining::Finished) {
            self.remaining.store(self.limit(), Ordering::Relaxed);
            self.last_update.set_millis(0);

            true
        } else {
            false
        }
    }

    /// Update this bucket's ratelimit data after a request has been made.
    /// The time of the request should be given.
    pub fn update(&self, ratelimits: &Present, time: u64) {
        let bucket_limit = self.limit();

        if self.last_update.is_empty() {
            debug!(millis = time, "updated the last update time");
            self.last_update.set_millis(time);
        }

        if bucket_limit != ratelimits.limit() && bucket_limit == u64::max_value() {
            self.reset_after
                .store(ratelimits.reset_after(), Ordering::SeqCst);
            self.limit.store(ratelimits.limit(), Ordering::SeqCst);
        }

        self.remaining
            .store(ratelimits.remaining(), Ordering::Relaxed);
    }

    /// Submits a ticket to the queue
    /// A oneshot receiver is returned and will be called when the ticket is accepted.
    pub fn ticket(&self) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.queue.push(tx);
        rx
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ops::Add,
        time::{Duration, Instant, SystemTime},
    };

    use tokio::time::timeout;
    use tracing::info;
    use twilight_http_ratelimiting::RatelimitHeaders;

    use super::Bucket;

    #[test_log::test(tokio::test)]
    async fn should_ratelimit() {
        let bucket = Bucket::new();

        // Intialize a bucket with one remaining ticket
        // and that resets in oue hour
        let mreset = SystemTime::now()
            .add(Duration::from_secs(100))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let headers: [(&str, &[u8]); 5] = [
            ("x-ratelimit-bucket", b"123"),
            ("x-ratelimit-limit", b"100"),
            ("x-ratelimit-remaining", b"1"),
            ("x-ratelimit-reset", mreset.as_bytes()),
            ("x-ratelimit-reset-after", b"100.000"),
        ];
        if let RatelimitHeaders::Present(present) =
            RatelimitHeaders::from_pairs(headers.into_iter()).unwrap()
        {
            // Integer truncating is expected
            #[allow(clippy::cast_possible_truncation)]
            bucket.update(
                &present,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            );
        }

        let ticket = bucket.ticket();

        info!("first request");
        // We should accept one ticket
        let respo = timeout(Duration::from_secs(10), ticket).await;
        assert!(respo.is_ok());

        info!("second request");

        let ticket = bucket.ticket();
        // We should accept one ticket
        let respo = timeout(Duration::from_secs(1), ticket).await;

        // the ticket should not have responded because the queue is locked
        assert!(respo.is_err());
    }

    #[test_log::test(tokio::test)]
    async fn should_block_until_possible() {
        let bucket = Bucket::new();

        // Intialize a bucket with one remaining ticket
        // and that resets in oue hour
        let mreset = SystemTime::now()
            .add(Duration::from_secs(100))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let headers: [(&str, &[u8]); 5] = [
            ("x-ratelimit-bucket", b"123"),
            ("x-ratelimit-limit", b"100"),
            ("x-ratelimit-remaining", b"0"),
            ("x-ratelimit-reset", mreset.as_bytes()),
            ("x-ratelimit-reset-after", b"10.000"),
        ];

        if let RatelimitHeaders::Present(present) =
            RatelimitHeaders::from_pairs(headers.into_iter()).unwrap()
        {
            // Integer truncating is expected
            #[allow(clippy::cast_possible_truncation)]
            bucket.update(
                &present,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            );
        }

        let ticket = bucket.ticket();
        let start = Instant::now();

        // in this case, the ratelimiter should wait 10 seconds
        let respo = timeout(Duration::from_secs(12), ticket).await;
        let end = start.elapsed().as_secs();

        // we should have waited 10 seconds (+- 1s)
        assert_eq!(10, end);
        // and the ticket should be a success
        assert!(respo.is_ok());
    }
}
