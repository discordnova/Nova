#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    unsafe_code
)]
#![allow(clippy::redundant_pub_crate)]
use async_nats::{Client, HeaderMap, HeaderValue};
use config::Gateway;
use leash::{AnyhowResultFuture, Component};
use opentelemetry::{global, propagation::Injector};
use shared::{
    config::Settings,
    payloads::{CachePayload, DispatchEventTagged},
};
use std::{convert::TryFrom, future::Future, pin::Pin, str::FromStr};
use tokio::{select, sync::oneshot};
use tokio_stream::StreamExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use twilight_gateway::{Event, Shard};
pub mod config;
use tracing::{debug, error, info, info_span, instrument, Instrument};
use twilight_model::gateway::event::DispatchEvent;

struct MetadataMap<'a>(&'a mut HeaderMap);

impl<'a> Injector for MetadataMap<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key, HeaderValue::from_str(&value).unwrap());
    }
}

pub struct GatewayServer {}

impl Component for GatewayServer {
    type Config = Gateway;
    const SERVICE_NAME: &'static str = "gateway";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        mut stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let (shard, mut events) = Shard::builder(settings.token.clone(), settings.intents)
                .shard(settings.shard, settings.shard_total)?
                .build();

            let nats = Into::<Pin<Box<dyn Future<Output = anyhow::Result<Client>> + Send>>>::into(
                settings.nats,
            )
            .await?;
            shard.start().await?;

            loop {
                select! {
                    event = events.next() => {
                        if let Some(event) = event {
                           let _ = handle_event(event, &nats)
                            .await
                            .map_err(|err| error!(error = ?err, "event publish failed"));
                        } else {
                            break
                        }
                    },
                    _ = (&mut stop) => break
                };
            }

            info!("stopping shard...");
            shard.shutdown();

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}

#[instrument]
async fn handle_event(event: Event, nats: &Client) -> anyhow::Result<()> {
    if let Event::Ready(ready) = event {
        info!(username = ready.user.name, "logged in");
    } else {
        let name = event.kind().name();
        if let Ok(dispatch_event) = DispatchEvent::try_from(event) {
            let name = name.unwrap();
            debug!(event_name = name, "handling dispatch event");

            let data = CachePayload {
                data: DispatchEventTagged(dispatch_event),
            };
            let value = serde_json::to_string(&data)?;
            let bytes = bytes::Bytes::from(value);

            let span = info_span!("nats send");

            let mut header_map = HeaderMap::new();
            let context = span.context();
            global::get_text_map_propagator(|propagator| {
                propagator.inject_context(&context, &mut MetadataMap(&mut header_map));
            });

            nats.publish_with_headers(format!("nova.cache.dispatch.{name}"), header_map, bytes)
                .instrument(info_span!("sending to nats"))
                .await?;
        }
    }

    Ok(())
}
