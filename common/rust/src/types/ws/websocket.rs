use crate::types::dispatch::presence_update::PresenceUpdate;
use crate::types::ws::RawJson;
use serde::de::{Deserialize, Deserializer, Error as SerdeDeError};
use serde::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Debug;
use super::{
     heartbeat::Heartbeat, heartbeat_ack::HeartbeatACK, hello::Hello,
    identify::Identify, invalid_session::InvalidSession,
    reconnect::Reconnect, request_guild_members::RequestGuildMembers, resume::Resume,
    voice_state_update::VoiceStateUpdate,
};
use crate::types::dispatch::Dispatch;

#[derive(serde::Deserialize, Debug, Serialize)]
#[serde(bound(deserialize = "T: Deserialize<'de> + Debug"))]
pub struct BasePacket<T> {
    #[serde(rename = "d")]
    pub data: T,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum WebsocketPacketType {
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

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub enum WebsocketPacket {
    Dispatch(Dispatch),
    Heartbeat(BasePacket<Heartbeat>),
    Identify(BasePacket<Identify>),
    PresenceUpdate(BasePacket<PresenceUpdate>),
    VoiceStateUpdate(BasePacket<VoiceStateUpdate>),
    Resume(BasePacket<Resume>),
    Reconnect(BasePacket<Reconnect>),
    RequestGuildMembers(BasePacket<RequestGuildMembers>),
    InvalidSession(BasePacket<InvalidSession>),
    Hello(BasePacket<Hello>),
    HeartbeatACK(BasePacket<HeartbeatACK>),
}

impl<'de> Deserialize<'de> for WebsocketPacket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let object = RawJson::deserialize(deserializer)?;

        let op = object
            .get("op")
            .ok_or_else(|| SerdeDeError::custom("missing opcode"))
            .and_then(WebsocketPacketType::deserialize)
            .map_err(SerdeDeError::custom)?;

        match op {
            WebsocketPacketType::Dispatch => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Dispatch)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::Heartbeat => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Heartbeat)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::Identify => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Identify)
                .map_err(SerdeDeError::custom), 
            WebsocketPacketType::PresenceUpdate => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::PresenceUpdate)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::VoiceStateUpdate => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::VoiceStateUpdate)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::Resume => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Resume)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::Reconnect => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Reconnect)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::RequestGuildMembers => {
                serde_json::from_value(Value::Object(object))
                    .map(WebsocketPacket::RequestGuildMembers)
                    .map_err(SerdeDeError::custom)
            }
            WebsocketPacketType::InvalidSession => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::InvalidSession)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::Hello => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::Hello)
                .map_err(SerdeDeError::custom),
            WebsocketPacketType::HeartbeatACK => serde_json::from_value(Value::Object(object))
                .map(WebsocketPacket::HeartbeatACK)
                .map_err(SerdeDeError::custom),
        }
    }
}
