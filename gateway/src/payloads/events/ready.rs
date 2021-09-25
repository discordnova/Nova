use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Ready {
    #[serde(rename = "v")]
    pub version: u64,
    pub user: Value,
    pub guilds: Vec<Value>,
    pub session_id: String,
    pub shard: Option<[i64;2]>,
    pub application: Value,
}
