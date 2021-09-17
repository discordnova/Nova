use self::state::SessionState;

use super::connection::Connection;
mod actions;
mod connection;
mod state;

/// Represents a shard & all the reconnection logic related to it
pub struct Shard {
    connection: Option<Connection>,
    state: SessionState,
}

impl Shard {
    /// Creates a new shard instance
    pub fn new() -> Self {
        Shard {
            connection: None,
            state: SessionState::default(),
        }
    }
}
