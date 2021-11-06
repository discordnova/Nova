use common::{
    config::Settings,
    log::{debug, info},
    nats_crate::Connection,
    payloads::{CachePayload, SerializeHelper, Tracing},
};
use config::Config;
use std::error::Error;
use twilight_gateway::{Event, Shard};
mod config;
use futures::StreamExt;

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
            Event::Resumed => {}
            Event::GatewayHeartbeat(_) => {}
            Event::GatewayHeartbeatAck => {}
            Event::GatewayInvalidateSession(_) => {}
            Event::GatewayReconnect => {}
            Event::GatewayHello(_) => {}

            Event::ShardConnected(_) => {}
            Event::ShardConnecting(_) => {}
            Event::ShardDisconnected(_) => {}
            Event::ShardIdentifying(_) => {}
            Event::ShardReconnecting(_) => {}
            Event::ShardPayload(_) => {}
            Event::ShardResuming(_) => {}

            _ => {
                let data = CachePayload {
                    tracing: Tracing {
                        node_id: "".to_string(),
                        span: None,
                    },
                    data: SerializeHelper(event),
                };
                let value = serde_json::to_string(&data)?;
                debug!("nats send: {}", value);
                nats.publish(
                    &format!("nova.cache.dispatch.{}", data.data.0.kind().name().unwrap()),
                    value,
                )?;
            }
        }
    }

    Ok(())
}
