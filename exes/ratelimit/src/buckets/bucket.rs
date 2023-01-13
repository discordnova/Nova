use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{sync::oneshot, task::JoinHandle};
use tracing::debug;
use twilight_http_ratelimiting::headers::Present;

use super::{async_queue::AsyncQueue, atomic_instant::AtomicInstant, redis_lock::RedisLock};

#[derive(Clone, Debug)]
pub enum TimeRemaining {
    Finished,
    NotStarted,
    Some(Duration),
}

#[derive(Debug)]
pub struct Bucket {
    pub limit: AtomicU64,
    /// Queue associated with this bucket.
    pub queue: AsyncQueue,
    /// Number of tickets remaining.
    pub remaining: AtomicU64,
    /// Duration after the [`Self::last_update`] time the bucket will refresh.
    pub reset_after: AtomicU64,
    /// When the bucket's ratelimit refresh countdown started (unix millis)
    pub last_update: AtomicInstant,

    pub tasks: Vec<JoinHandle<()>>,
}

impl Drop for Bucket {
    fn drop(&mut self) {
        for join in &self.tasks {
            join.abort();
        }
    }
}

impl Bucket {
    /// Create a new bucket for the specified [`Path`].
    pub fn new(global: Arc<RedisLock>) -> Arc<Self> {
        let tasks = vec![];

        let this = Arc::new(Self {
            limit: AtomicU64::new(u64::max_value()),
            queue: AsyncQueue::default(),
            remaining: AtomicU64::new(u64::max_value()),
            reset_after: AtomicU64::new(u64::max_value()),
            last_update: AtomicInstant::empty(),
            tasks,
        });

        // Run with 4 dequeue tasks
        for _ in 0..4 {
            let this = this.clone();
            let global = global.clone();
            tokio::spawn(async move {
                while let Some(element) = this.queue.pop().await {
                    // we need to wait
                    if let Some(duration) = global.locked_for().await {
                        tokio::time::sleep(duration).await;
                    }

                    if this.remaining() == 0 {
                        debug!("0 tickets remaining, we have to wait.");

                        match this.time_remaining() {
                            TimeRemaining::Finished => {
                                this.try_reset();
                            }
                            TimeRemaining::Some(duration) => {
                                debug!(milliseconds=%duration.as_millis(), "waiting for ratelimit");
                                tokio::time::sleep(duration).await;

                                this.try_reset();
                            }
                            TimeRemaining::NotStarted => {}
                        }
                    }

                    this.remaining.fetch_sub(1, Ordering::Relaxed);
                    let _ = element
                        .send(())
                        .map_err(|_| debug!("response channel was closed."));
                }
            });
        }

        this
    }

    /// Total number of tickets allotted in a cycle.
    pub fn limit(&self) -> u64 {
        self.limit.load(Ordering::Relaxed)
    }

    /// Number of tickets remaining.
    pub fn remaining(&self) -> u64 {
        self.remaining.load(Ordering::Relaxed)
    }

    /// Duration after the [`started_at`] time the bucket will refresh.
    ///
    /// [`started_at`]: Self::started_at
    pub fn reset_after(&self) -> u64 {
        self.reset_after.load(Ordering::Relaxed)
    }

    /// Time remaining until this bucket will reset.
    pub fn time_remaining(&self) -> TimeRemaining {
        let reset_after = self.reset_after();
        let last_update = &self.last_update;

        if last_update.is_empty() {
            let elapsed = last_update.elapsed();

            if elapsed > Duration::from_millis(reset_after) {
                return TimeRemaining::Finished;
            }

            TimeRemaining::Some(Duration::from_millis(reset_after) - elapsed)
        } else {
            TimeRemaining::NotStarted
        }
    }

    /// Try to reset this bucket's [`started_at`] value if it has finished.
    ///
    /// Returns whether resetting was possible.
    ///
    /// [`started_at`]: Self::started_at
    pub fn try_reset(&self) -> bool {
        if self.last_update.is_empty() {
            return false;
        }

        if let TimeRemaining::Finished = self.time_remaining() {
            self.remaining.store(self.limit(), Ordering::Relaxed);
            self.last_update.set_millis(0);

            true
        } else {
            false
        }
    }

    /// Update this bucket's ratelimit data after a request has been made.
    pub fn update(&self, ratelimits: Present, time: u64) {
        let bucket_limit = self.limit();

        if self.last_update.is_empty() {
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

    pub async fn ticket(&self) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.queue.push(tx);
        rx
    }
}
