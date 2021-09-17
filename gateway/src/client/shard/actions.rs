use futures::SinkExt;
use log::error;
use serde_json::Value;

use crate::client::payloads::message::{MessageBase, OpCodes};

use super::Shard;

/// Implement the available actions for nova in the gateway.
impl Shard {
    /// Updates the presence of the current shard.
    #[allow(dead_code)]
    pub async fn presence_update(&mut self) -> Result<(), ()> {
        if let Some(connection) = &mut self.connection {
            connection
                .send(MessageBase {
                    t: None,
                    s: None,
                    op: OpCodes::PresenceUpdate,
                    // todo: proper payload for this
                    d: Value::Null,
                })
                .await?;
        } else {
            error!("the connection is not open")
        }
        Ok(())
    }
    /// Updates the voice status of the current shard in a certain channel.
    #[allow(dead_code)]
    pub async fn voice_state_update(&mut self) -> Result<(), ()> {
        if let Some(connection) = &mut self.connection {
            connection
                .send(MessageBase {
                    t: None,
                    s: None,
                    op: OpCodes::VoiceStateUpdate,
                    // todo: proper payload for this
                    d: Value::Null,
                })
                .await?;
        } else {
            error!("the connection is not open")
        }
        Ok(())
    }
    /// Ask discord for more informations about offline guild members.
    #[allow(dead_code)]
    pub async fn request_guild_members(&mut self) -> Result<(), ()> {
        if let Some(connection) = &mut self.connection {
            connection
                .send(MessageBase {
                    t: None,
                    s: None,
                    op: OpCodes::RequestGuildMembers,
                    // todo: proper payload for this
                    d: Value::Null,
                })
                .await?;
        } else {
            error!("the connection is not open")
        }
        Ok(())
    }
}
