use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    #[serde(rename = "heartbeat_interval")]
    pub heartbeat_interval: u64
}
