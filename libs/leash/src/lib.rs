use anyhow::Result;
use serde::de::DeserializeOwned;
use shared::config::Settings;
use std::{future::Future, pin::Pin};
use tokio::sync::oneshot;

pub type AnyhowResultFuture<T> = Pin<Box<dyn Future<Output = Result<T>> + Send>>;
pub trait Component: Send + Sync + 'static + Sized {
    type Config: Default + Clone + DeserializeOwned + Send;

    const SERVICE_NAME: &'static str;
    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()>;
    fn new() -> Self;

    fn _internal_start(self) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            pretty_env_logger::init();
            let settings = Settings::<Self::Config>::new(Self::SERVICE_NAME);
            let (stop, stop_channel) = oneshot::channel();

            // Start the grpc healthcheck
            tokio::spawn(async move {});

            // Start the prometheus monitoring job
            tokio::spawn(async move {});

            tokio::spawn(async move {
                #[cfg(unix)]
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .unwrap()
                    .recv()
                    .await;
                #[cfg(not(unix))]
                tokio::signal::ctrl_c().await;

                stop.send(()).unwrap();
            });
            self.start(settings?, stop_channel).await
        })
    }
}

#[macro_export]
macro_rules! ignite {
    ($c:ty) => {
        #[allow(dead_code)]
        fn main() -> anyhow::Result<()> {
            use leash::Component;
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(<$c as Component>::new()._internal_start())?;
            Ok(())
        }
    };
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use tokio::sync::oneshot;

    use crate as leash;

    #[derive(Clone, Copy)]
    struct TestComponent {}

    #[derive(Default, Clone, Deserialize, Copy)]
    struct TestComponentConfig {}

    impl leash::Component for TestComponent {
        type Config = TestComponentConfig;
        const SERVICE_NAME: &'static str = "test_component";

        fn start(
            &self,
            _settings: shared::config::Settings<Self::Config>,
            _stop: oneshot::Receiver<()>,
        ) -> crate::AnyhowResultFuture<()> {
            Box::pin(async move { Ok(()) })
        }

        fn new() -> Self {
            Self {}
        }
    }

    ignite!(TestComponent);
}
