use super::{dispatch::Dispatch, opcodes::{OpCodes, hello::Hello}};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde::de::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de> + std::fmt::Debug"))]
pub struct BaseMessage<T> {
    pub t: Option<String>,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    pub op: OpCodes,
    #[serde(rename = "d")]
    pub data: T,
}

#[derive(Debug)]
pub enum Message {
    Dispatch(BaseMessage<Dispatch>),
    Reconnect(BaseMessage<()>),
    InvalidSession(BaseMessage<bool>),
    Hello(BaseMessage<Hello>),
    HeartbeatACK(BaseMessage<()>),
}

impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> where D::Error : Error {
        let value = Value::deserialize(d)?;
        let val = value.get("op").and_then(Value::as_u64).unwrap();

        if let Some(op) = num::FromPrimitive::from_u64(val) {
            match op {
                OpCodes::Dispatch => {
                    // todo: remove unwrap
                    let t = Some(value.get("t").unwrap().to_string());
                    let sequence = value.get("s").unwrap().as_u64();

                    // we need to find a better solution than clone
                    match serde_json::from_value(value) {
                        Ok(data) => {
                            Ok(Message::Dispatch(BaseMessage {
                                op,
                                t,
                                sequence,
                                data
                            }))
                        },
                        Err(e) => Err(Error::custom(e)),
                    }
                },
                
                OpCodes::Reconnect => {
                    match BaseMessage::deserialize(value) {
                        Ok(data) => Ok(Message::Reconnect(data)),
                        Err(e) => Err(Error::custom(e)),
                    }
                },
                OpCodes::InvalidSession => {
                    match BaseMessage::deserialize(value) {
                        Ok(data) => Ok(Message::InvalidSession(data)),
                        Err(e) => Err(Error::custom(e)),
                    }
                },
                OpCodes::Hello => {
                    match BaseMessage::deserialize(value) {
                        Ok(data) => Ok(Message::Hello(data)),
                        Err(e) => Err(Error::custom(e)),
                    }
                },
                OpCodes::HeartbeatACK => {
                    match BaseMessage::deserialize(value) {
                        Ok(data) => Ok(Message::HeartbeatACK(data)),
                        Err(e) => Err(Error::custom(e)),
                    }
                },
                _ => panic!("Cannot convert"),
            }
        } else {
            Err(Error::custom("unknown opcode"))
        }
    }
}
