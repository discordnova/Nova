use serde::{Deserialize, Serialize};

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
    pub intents: u16,
    pub properties: IdentifyProprerties,
    pub shard: Option<[i64; 2]>,
}