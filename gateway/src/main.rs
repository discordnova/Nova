use std::sync::Arc;

use common::{config::Settings, nats_crate::Connection};
use shard::{Shard, ShardConfig};
extern crate num_derive;

pub mod connection;
mod error;
mod utils;
mod shard;



#[tokio::main]
async fn main() {
    let settings: Settings<ShardConfig> = Settings::new("gateway").unwrap();
    let _guard = settings.sentry();
    let nats: Arc<Connection> = Arc::new(settings.nats.into());

    let mut shard = Shard::new(settings.config, nats);
    shard.start().await;
}
