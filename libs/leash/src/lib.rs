use anyhow::Result;
use serde::de::DeserializeOwned;
use shared::config::Settings;
use std::{future::Future, pin::Pin};

pub type AnyhowResultFuture<T> = Pin<Box<dyn Future<Output = Result<T>>>>;
pub trait Component: Send + Sync + 'static + Sized {
    type Config: Default + Clone + DeserializeOwned;

    const SERVICE_NAME: &'static str;
    fn start(&self, settings: Settings<Self::Config>) -> AnyhowResultFuture<()>;
    fn new() -> Self;

    fn _internal_start(self) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let settings = Settings::<Self::Config>::new(Self::SERVICE_NAME);

            // Start the grpc healthcheck
            tokio::spawn(async move {});

            // Start the prometheus monitoring job
            tokio::spawn(async move {});

            self.start(settings?).await
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
        ) -> crate::AnyhowResultFuture<()> {
            Box::pin(async move { Ok(()) })
        }

        fn new() -> Self {
            Self {}
        }
    }
    
    ignite!(TestComponent);
}
