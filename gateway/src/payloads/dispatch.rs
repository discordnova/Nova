use serde::{Deserialize, Deserializer, Serialize};

use serde_json::Value;

use super::gateway::BaseMessage;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ready {
    #[serde(rename = "v")]
    pub version: u64,
    pub user: Value,
    pub guilds: Vec<Value>,
    pub session_id: String,
    pub shard: Option<[i64;2]>,
    pub application: Value,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum FakeDispatch {
    #[serde(rename = "READY")]
    Ready(Ready),
    Other(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dispatch {
    Ready(Ready),
    Other(BaseMessage<Value>)
}

impl<'de> Deserialize<'de> for Dispatch {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // todo: error handling
        let value = Value::deserialize(d)?;

        if value.get("t").unwrap() == "READY" {
            Ok(Dispatch::Ready(Ready::deserialize(value.get("d").unwrap()).unwrap()))
        } else {
            Ok(Dispatch::Other(BaseMessage::deserialize(value).unwrap()))
        }
    }
}