use log::info;
use serde::{Deserialize, Deserializer};

use serde_json::Value;

use super::{events::ready::Ready, opcodes::OpCodes};

/// Represents an unknown event not handled by the gateway itself
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UnknownDispatch {
    pub t: String,
    pub d: Value,
    pub s: i64,
    pub op: OpCodes,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "t", content = "d")]
#[serde(remote = "Dispatch")]
pub enum Dispatch {
    #[serde(rename = "READY")]
    Ready(Ready),
    #[serde(rename = "RESUMED")]
    Resumed(()),

    #[serde(skip_deserializing)]
    Other(UnknownDispatch),
}

impl<'de> Deserialize<'de> for Dispatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        info!("hey");
        let s = UnknownDispatch::deserialize(deserializer)?;
        Ok(Self::Other(s))
    }
}
