use serde::{Serialize, Deserialize};

/// The first message sent by the gateway to initialize the heartbeating
#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    #[serde(rename = "heartbeat_interval")]
    pub heartbeat_interval: u64
}