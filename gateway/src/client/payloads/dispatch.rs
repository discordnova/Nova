use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ready {
    #[serde(rename = "v")]
    version: u64,
    user: Value,
    guilds: Vec<Value>,
    session_id: String,
    shard: Option<[i64;2]>,
    application: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "t", content = "d")]
pub enum Dispatch {
    #[serde(rename = "READY")]
    Ready(Ready),
}