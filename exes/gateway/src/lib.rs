use async_nats::{Client, HeaderMap, HeaderValue};
use config::GatewayConfig;
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
use tracing::{debug, info, trace_span};
use twilight_model::gateway::event::DispatchEvent;

struct MetadataMap<'a>(&'a mut HeaderMap);

impl<'a> Injector for MetadataMap<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key, HeaderValue::from_str(&value).unwrap())
    }
}

pub struct GatewayServer {}

impl Component for GatewayServer {
    type Config = GatewayConfig;
    const SERVICE_NAME: &'static str = "gateway";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        mut stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let (shard, mut events) = Shard::builder(settings.token.to_owned(), settings.intents)
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
                            match event {
                                Event::Ready(ready) => {
                                    info!("Logged in as {}", ready.user.name);
                                },

                                _ => {

                                    let name = event.kind().name();
                                    if let Ok(dispatch_event) = DispatchEvent::try_from(event) {
                                        debug!("handling event {}", name.unwrap());

                                        let data = CachePayload {
                                            data: DispatchEventTagged {
                                                data: dispatch_event,
                                            },
                                        };
                                        let value = serde_json::to_string(&data)?;
                                        let bytes = bytes::Bytes::from(value);

                                        let span = trace_span!("nats send");

                                        let mut header_map = HeaderMap::new();
                                        let context = span.context();
                                        global::get_text_map_propagator(|propagator| {
                                            propagator.inject_context(&context, &mut MetadataMap(&mut header_map))
                                        });

                                        nats.publish_with_headers(format!("nova.cache.dispatch.{}", name.unwrap()), header_map, bytes)
                                            .await?;
                                    }
                                }
                            }
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
