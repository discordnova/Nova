use super::GlobalLock;

pub struct NoOpLock;
impl GlobalLock for NoOpLock {
    fn lock_for<'a>(
        self: &'a std::sync::Arc<Self>,
        _duration: std::time::Duration,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {})
    }

    fn is_locked<'a>(
        self: &'a std::sync::Arc<Self>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<std::time::Duration>> + Send + 'a>>
    {
        Box::pin(async move { None })
    }
}
