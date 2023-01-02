use anyhow::Result;
use serde::de::DeserializeOwned;
use shared::{
    config::Settings,
    log::{error, info},
};
use std::{future::Future, pin::Pin};
use tokio::{signal::{unix::SignalKind}, sync::oneshot};

pub type AnyhowResultFuture<T> = Pin<Box<dyn Future<Output = Result<T>>>>;
pub trait Component: Send + Sync + 'static + Sized {
    type Config: Default + Clone + DeserializeOwned;

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
                match tokio::signal::unix::signal(SignalKind::terminate()).unwrap().recv().await {
                    Some(()) => {
                        info!("Stopping program.");

                        stop.send(()).unwrap();
                    }
                    None => {
                        error!("Unable to listen for shutdown signal");
                        // we also shut down in case of error
                    }
                }
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
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(Box::new(<$c as Component>::new())._internal_start())?;
            Ok(())
        }
    };
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use tokio::sync::oneshot;

    use crate::Component;

    #[derive(Clone, Copy)]
    struct TestComponent {}

    #[derive(Default, Clone, Deserialize, Copy)]
    struct TestComponentConfig {}

    impl Component for TestComponent {
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
