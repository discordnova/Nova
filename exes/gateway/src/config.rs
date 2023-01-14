use serde::{Deserialize, Serialize};
use twilight_gateway::Intents;

#[derive(Serialize, Deserialize, Clone)]
pub struct Gateway {
    pub token: String,
    pub intents: Intents,
    pub shard: u64,
    pub shard_total: u64,
}

impl Default for Gateway {
    fn default() -> Self {
        Self {
            intents: Intents::empty(),
            token: String::default(),
            shard_total: 1,
            shard: 1,
        }
    }
}
