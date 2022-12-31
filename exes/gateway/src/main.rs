use config::Config;
use shared::{
    config::Settings,
    log::{debug, info},
    nats_crate::Connection,
    payloads::{CachePayload, DispatchEventTagged, Tracing},
};
use std::{convert::TryFrom, error::Error};
use twilight_gateway::{Event, Shard};
mod config;
use futures::StreamExt;
use twilight_model::gateway::event::DispatchEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let settings: Settings<Config> = Settings::new("gateway").unwrap();
    let (shard, mut events) = Shard::new(settings.config.token, settings.config.intents);
    let nats: Connection = settings.nats.into();

    shard.start().await?;

    while let Some(event) = events.next().await {
        match event {
            Event::Ready(ready) => {
                info!("Logged in as {}", ready.user.name);
            }

            _ => {
                let name = event.kind().name().unwrap();
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
                    nats.publish(&format!("nova.cache.dispatch.{}", name), value)?;
                }
            }
        }
    }

    Ok(())
}
