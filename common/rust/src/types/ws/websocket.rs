use super::{
    heartbeat::Heartbeat, hello::Hello, identify::Identify, invalid_session::InvalidSession,
    reconnect::Reconnect, request_guild_members::RequestGuildMembers, resume::Resume,
    voice_state_update::VoiceStateUpdate,
};
use crate::types::dispatch::presence_update::PresenceUpdate;
use crate::types::dispatch::Dispatch;
use crate::types::dispatch::DispatchType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Debug;

/// Represents the list of possible Opcodes in the
/// discord gateway.
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

/// Represents a raw packet returned by the gateway
/// This can be deserialized into a WebsocketPacket
/// using the From<T> trait.
#[derive(Deserialize, Debug, Serialize)]
pub struct BasePacket {
    #[serde(rename = "d")]
    pub data: Option<Value>,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    #[serde(rename = "op")]
    pub operation: WebsocketPacketType,
    #[serde(rename = "t")]
    pub type_: Option<DispatchType>,
}

impl From<WebsocketPacket> for Result<BasePacket, serde_json::Error> {
    fn from(packet: WebsocketPacket) -> Self {
        Ok(BasePacket {
            operation: match packet {
                WebsocketPacket::Dispatch(_) => WebsocketPacketType::Dispatch,
                WebsocketPacket::Heartbeat(_) => WebsocketPacketType::Heartbeat,
                WebsocketPacket::Identify(_) => WebsocketPacketType::Identify,
                WebsocketPacket::PresenceUpdate(_) => WebsocketPacketType::PresenceUpdate,
                WebsocketPacket::VoiceStateUpdate(_) => WebsocketPacketType::VoiceStateUpdate,
                WebsocketPacket::Resume(_) => WebsocketPacketType::Resume,
                WebsocketPacket::Reconnect(_) => WebsocketPacketType::Reconnect,
                WebsocketPacket::RequestGuildMembers(_) => WebsocketPacketType::RequestGuildMembers,
                WebsocketPacket::InvalidSession(_) => WebsocketPacketType::InvalidSession,
                WebsocketPacket::Hello(_) => WebsocketPacketType::Hello,
                WebsocketPacket::HeartbeatACK(_) => WebsocketPacketType::HeartbeatACK,
            },
            data: Some(packet.value()?),
            sequence: None,
            type_: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "t", content = "c")]
pub enum WebsocketPacket {
    Dispatch(Box<Dispatch>),
    Heartbeat(Heartbeat),
    Identify(Identify),
    PresenceUpdate(PresenceUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    Resume(Resume),
    Reconnect(Reconnect),
    RequestGuildMembers(RequestGuildMembers),
    InvalidSession(InvalidSession),
    Hello(Hello),
    HeartbeatACK(()),
}

impl WebsocketPacket {
    pub fn value(&self) -> Result<serde_json::Value, serde_json::Error> {
        match self {
            WebsocketPacket::Dispatch(v) => serde_json::to_value(v),
            WebsocketPacket::Heartbeat(v) => serde_json::to_value(v),
            WebsocketPacket::Identify(v) => serde_json::to_value(v),
            WebsocketPacket::PresenceUpdate(v) => serde_json::to_value(v),
            WebsocketPacket::VoiceStateUpdate(v) => serde_json::to_value(v),
            WebsocketPacket::Resume(v) => serde_json::to_value(v),
            WebsocketPacket::Reconnect(v) => serde_json::to_value(v),
            WebsocketPacket::RequestGuildMembers(v) => serde_json::to_value(v),
            WebsocketPacket::InvalidSession(v) => serde_json::to_value(v),
            WebsocketPacket::Hello(v) => serde_json::to_value(v),
            WebsocketPacket::HeartbeatACK(v) => serde_json::to_value(v),
        }
    }
}

impl From<BasePacket> for Result<WebsocketPacket, serde_json::Error> {
    fn from(packet: BasePacket) -> Self {
        Ok(match packet.operation {
            WebsocketPacketType::Dispatch => {
                WebsocketPacket::Dispatch(Box::new(Result::<Dispatch, serde_json::Error>::from(
                    packet,
                )?))
            }
            WebsocketPacketType::Heartbeat => {
                WebsocketPacket::Heartbeat(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::Identify => {
                WebsocketPacket::Identify(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::PresenceUpdate => {
                WebsocketPacket::PresenceUpdate(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::VoiceStateUpdate => {
                WebsocketPacket::VoiceStateUpdate(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::Resume => {
                WebsocketPacket::Resume(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::Reconnect => {
                WebsocketPacket::Reconnect(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::RequestGuildMembers => {
                WebsocketPacket::RequestGuildMembers(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::InvalidSession => {
                WebsocketPacket::InvalidSession(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::Hello => {
                WebsocketPacket::Hello(serde_json::from_value(packet.data.unwrap())?)
            }
            WebsocketPacketType::HeartbeatACK => WebsocketPacket::HeartbeatACK(()),
        })
    }
}
