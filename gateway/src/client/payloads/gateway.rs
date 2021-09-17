use super::dispatch::Dispatch;
use super::payloads::hello::Hello;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

macro_rules! num_to_enum {
    ($num:expr => $enm:ident<$tpe:ty>{ $($fld:ident),+ }; $err:expr) => ({
        match $num {
            $(_ if $num == $enm::$fld as $tpe => { $enm::$fld })+
            _ => $err
        }
    });
}

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FullMessage<T> {
    #[serde(rename = "d")]
    pub dispatch_type: Option<String>,
    #[serde(rename = "s")]
    pub sequence: Option<OpCodes>,
    pub op: OpCodes,
    #[serde(rename = "d")]
    pub data: T,
}

pub enum Message {
    Dispatch(FullMessage<Dispatch>),
    Reconnect(FullMessage<()>),
    InvalidSession(FullMessage<bool>),
    Hello(FullMessage<Hello>),
    HeartbeatACK(FullMessage<()>),
}

impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        let val = value.get("op").and_then(Value::as_u64).unwrap();
        let op_code = num_to_enum!(
            val => OpCodes<u64>{
                Dispatch,
                Heartbeat,
                Identify,
                PresenceUpdate,
                VoiceStateUpdate,
                Resume,
                Reconnect,
                RequestGuildMembers,
                InvalidSession,
                Hello,
                HeartbeatACK
            };
            panic!("Cannot convert number to `MyEnum`")
        );

        match op_code {
            OpCodes::Dispatch => Ok(Message::Dispatch(FullMessage::deserialize(value).unwrap())),
            OpCodes::Reconnect => Ok(Message::Reconnect(FullMessage::deserialize(value).unwrap())),
            OpCodes::InvalidSession => Ok(Message::InvalidSession(
                FullMessage::deserialize(value).unwrap(),
            )),
            OpCodes::Hello => Ok(Message::Hello(FullMessage::deserialize(value).unwrap())),
            OpCodes::HeartbeatACK => Ok(Message::HeartbeatACK(
                FullMessage::deserialize(value).unwrap(),
            )),
            _ => panic!("Cannot convert"),
        }
    }
}
