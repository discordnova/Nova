use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Deserialize, Serialize};


#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum OpCodes {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}

#[derive(Serialize, Deserialize)]
pub enum Dispatch {
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "RESUMED")]
    Resumed,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MessageBase {
    pub t: Option<String>,
    pub s: Option<i64>,
    pub op: OpCodes,
    pub d: Value
}
