use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

/// Simple async fifo (first in fist out) queue based on unbounded channels
/// 
/// # Usage
/// ```
/// # use ratelimit::buckets::async_queue::AsyncQueue;
/// # tokio_test::block_on(async {
/// let queue = AsyncQueue::<i64>::default();
/// // Pushing into the queue is syncronous
/// queue.push(123);
///     
/// // Popping from the queue is asyncronous
/// let value = queue.pop().await;
///     
/// // Our value should be the same!
/// assert_eq!(value, Some(123));
/// # });
/// ```
#[derive(Debug)]
pub struct AsyncQueue<T: Send> {
    rx: Mutex<UnboundedReceiver<T>>,
    tx: UnboundedSender<T>,
}

impl<T: Send> AsyncQueue<T> {
    /// Add a new item to the queue
    pub fn push(&self, tx: T) {
        let _sent = self.tx.send(tx);
    }

    /// Receive the first incoming ratelimit request.
    pub async fn pop(&self) -> Option<T> {
        let mut rx = self.rx.lock().await;

        rx.recv().await
    }
}

impl<T: Send> Default for AsyncQueue<T> {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            rx: Mutex::new(rx),
            tx,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buckets::async_queue::AsyncQueue;

    #[test_log::test(tokio::test)]
    async fn should_queue_dequeue_fifo() {
        let queue = AsyncQueue::<i64>::default();

        // queue data
        for i in 0..2_000_000 {
            queue.push(i);
        }

        for i in 0..2_000_000 {
            let result = queue.pop().await.unwrap();
            assert_eq!(i, result);
        }
    }
}
