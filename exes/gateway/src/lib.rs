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
use tracing_opentelemetry::OpenTelemetrySpanExt;
use twilight_gateway::{Event, Shard, ShardId};
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
            let mut shard = Shard::new(
                ShardId::new(settings.shard, settings.shard_total),
                settings.token.clone(),
                settings.intents,
            );

            let nats = Into::<Pin<Box<dyn Future<Output = anyhow::Result<Client>> + Send>>>::into(
                settings.nats,
            )
            .await?;

            loop {
                select! {
                    event = shard.next_event() => {
                        match event {
                            Ok(event) => {
                                let _ = handle_event(event, &nats)
                                    .await
                                    .map_err(|err| error!(error = ?err, "event publish failed"));
                            },
                            Err(source) => {
                                if source.is_fatal() {
                                    break;
                                }
                                continue;
                            }
                        }
                    },
                    _ = (&mut stop) => break
                };
            }

            info!("stopping shard...");
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
