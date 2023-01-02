use config::GatewayConfig;
use leash::{ignite, AnyhowResultFuture, Component};
use shared::{
    config::Settings,
    log::{debug, info},
    nats_crate::Client,
    payloads::{CachePayload, DispatchEventTagged, Tracing},
};
use tokio::sync::oneshot;
use std::{convert::TryFrom, pin::Pin};
use twilight_gateway::{Event, Shard};
mod config;
use futures::{Future, StreamExt, select};
use twilight_model::gateway::event::DispatchEvent;
use futures::FutureExt;

struct GatewayServer {}
impl Component for GatewayServer {
    type Config = GatewayConfig;
    const SERVICE_NAME: &'static str = "gateway";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let (shard, mut events) = Shard::builder(settings.token.to_owned(), settings.intents)
                .shard(settings.shard, settings.shard_total)?
                .build();

            let nats =
                Into::<Pin<Box<dyn Future<Output = anyhow::Result<Client>>>>>::into(settings.nats)
                    .await?;

            shard.start().await?;

            let mut stop = stop.fuse();
            loop {

                select! {
                    event = events.next().fuse() => {
                        if let Some(event) = event {
                            match event {
                                Event::Ready(ready) => {
                                    info!("Logged in as {}", ready.user.name);
                                }
            
                                _ => {
                                    let name = event.kind().name();
                                    if let Ok(dispatch_event) = DispatchEvent::try_from(event) {
                                        let data = CachePayload {
                                            tracing: Tracing {
                                                node_id: "".to_string(),
                                                span: None,
                                            },
                                            data: DispatchEventTagged {
                                                data: dispatch_event,
                                            },
                                        };
                                        let value = serde_json::to_string(&data)?;
                                        debug!("nats send: {}", value);
                                        let bytes = bytes::Bytes::from(value);
                                        nats.publish(format!("nova.cache.dispatch.{}", name.unwrap()), bytes)
                                            .await?;
                                    }
                                }
                            }
                        } else {
                            break
                        }
                    },
                    _ = stop => break
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

ignite!(GatewayServer);
