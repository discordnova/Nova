use serde_json::{Map, Value};
pub mod websocket;
pub mod heartbeat;
pub mod identify;
pub mod voice_state_update;
pub mod resume;
pub mod reconnect;
pub mod request_guild_members;
pub mod invalid_session;
pub mod hello;
pub mod heartbeat_ack;

pub type RawJson = Map<String, Value>;