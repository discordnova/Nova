#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
)]

use anyhow::Result;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace::{self};
use opentelemetry::sdk::Resource;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use serde::de::DeserializeOwned;
use shared::config::Settings;
use std::str::FromStr;
use std::{future::Future, pin::Pin};
use tokio::sync::oneshot;
use tracing::{info, log::trace};
use tracing_subscriber::filter::Directive;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
            global::set_text_map_propagator(TraceContextPropagator::new());
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_trace_config(trace::config().with_resource(Resource::new(vec![
                    KeyValue::new("service.name", Self::SERVICE_NAME),
                ])))
                .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
                .install_batch(opentelemetry::runtime::Tokio)?;

            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

            tracing_subscriber::registry()
                .with(fmt::layer())
                .with(telemetry)
                .with(
                    EnvFilter::builder()
                        .with_default_directive(Directive::from_str("info").unwrap())
                        .from_env()?,
                )
                .init();

            info!("Starting nova");
            let settings = Settings::<Self::Config>::new(Self::SERVICE_NAME);
            let (stop, stop_channel) = oneshot::channel();

            tokio::spawn(async move {
                trace!("started signal watching");
                #[cfg(unix)]
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .unwrap()
                    .recv()
                    .await;
                #[cfg(not(unix))]
                return tokio::signal::ctrl_c().await.unwrap();

                stop.send(()).unwrap();
            });

            trace!(
                "Starting component {component}",
                component = Self::SERVICE_NAME
            );
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
