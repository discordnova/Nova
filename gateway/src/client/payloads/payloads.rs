use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    #[serde(rename = "heartbeat_interval")]
    pub heartbeat_interval: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatACK {}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProprerties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identify {
    pub token: String,
    pub intents: i64,
    pub properties: IdentifyProprerties,
}