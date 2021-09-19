use common::config::Settings;
use shard::{Shard, ShardConfig};
#[macro_use]
extern crate num_derive;

pub mod connection;
mod error;
mod utils;
mod shard;
mod payloads;



#[tokio::main]
async fn main() {
    let settings: Settings<ShardConfig> = Settings::new("gateway").unwrap();
    let mut shard = Shard::new(settings.config);
    shard.start().await;
}
