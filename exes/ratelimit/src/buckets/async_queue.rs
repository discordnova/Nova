use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot::Sender,
    Mutex,
};

/// Queue of ratelimit requests for a bucket.
#[derive(Debug)]
pub struct AsyncQueue {
    /// Receiver for the ratelimit requests.
    rx: Mutex<UnboundedReceiver<Sender<()>>>,
    /// Sender for the ratelimit requests.
    tx: UnboundedSender<Sender<()>>,
}

impl AsyncQueue {
    /// Add a new ratelimit request to the queue.
    pub fn push(&self, tx: Sender<()>) {
        let _sent = self.tx.send(tx);
    }

    /// Receive the first incoming ratelimit request.
    pub async fn pop(&self) -> Option<Sender<()>> {
        let mut rx = self.rx.lock().await;

        rx.recv().await
    }
}

impl Default for AsyncQueue {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            rx: Mutex::new(rx),
            tx,
        }
    }
}
