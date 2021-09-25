use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use crate::{connection::Connection, payloads::opcodes::identify::Intents};
use self::state::{ConnectionState, SessionState};
mod actions;
mod connection;
mod state;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Sharding {
    pub total_shards: i64,
    pub current_shard: i64
}


#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ShardConfig {
    pub max_reconnects: usize,
    pub reconnect_delay_growth_factor: f32,
    pub reconnect_delay_minimum: usize,
    pub reconnect_delay_maximum: usize,
    pub token: String,
    
    pub large_threshold: Option<u64>,
    pub shard: Option<Sharding>,
    pub intents: BitFlags<Intents>
}

struct ConnectionWithState {
    conn: Connection,
    state: ConnectionState,
}

/// Represents a shard & all the reconnection logic related to it
pub struct Shard {
    connection: Option<ConnectionWithState>,
    state: Option<SessionState>,
    config: ShardConfig
}

impl Shard {
    /// Creates a new shard instance
    pub fn new(config: ShardConfig) -> Self {
        Shard {
            connection: None,
            state: None,
            config,
        }
    }
}
