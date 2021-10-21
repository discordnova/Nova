use std::env;

use common::{
    log::{debug, error, info},
    types::{
        dispatch::presence_update::PresenceUpdate,
        ws::{
            identify::{Identify, IdentifyProprerties},
            request_guild_members::RequestGuildMembers,
            resume::Resume,
            voice_state_update::VoiceStateUpdate,
            websocket::{BasePacket, WebsocketPacket},
        },
    },
};
use futures::SinkExt;

use crate::error::GatewayError;

use super::Shard;

/// Implement the available actions for nova in the gateway.
impl Shard {
    /// sends a message through the websocket
    pub async fn _send(&mut self, message: WebsocketPacket) -> Result<(), GatewayError> {
        debug!("Sending message {:?}", message);
        if let Some(connection) = &mut self.connection {
            if let Err(e) = connection.conn.send(&Result::<BasePacket, serde_json::Error>::from(message).unwrap()).await {
                error!("failed to send message {:?}", e);
                Err(GatewayError::from(e))
            } else {
                Ok(())
            }
        } else {
            Err(GatewayError::from("no open connection".to_string()))
        }
    }

    pub async fn _identify(&mut self) -> Result<(), GatewayError> {
        if let Some(state) = self.state.clone() {
            info!("Using session");
            self._send(WebsocketPacket::Resume(Resume {
                token: self.config.token.clone(),
                seq: state.sequence,
                session_id: state.session_id.clone(),
            }))
            .await
        } else {
            info!("Sending login");
            let mut shards: Option<[u64; 2]> = None;
            if let Some(sharding) = self.config.shard.as_ref() {
                shards = Some([sharding.current_shard, sharding.total_shards]);
            }
            self._send(WebsocketPacket::Identify(Identify {
                token: self.config.token.clone(),
                intents: self.config.intents,
                properties: IdentifyProprerties {
                    os: env::consts::OS.to_string(),
                    browser: "Nova".to_string(),
                    device: "Nova".to_string(),
                },
                shard: shards,
                compress: Some(false),
                large_threshold: Some(500),
                presence: None,
            }))
            .await
        }
    }

    pub async fn _disconnect(&mut self) {}

    /// Updates the presence of the current shard.
    #[allow(dead_code)]
    pub async fn presence_update(&mut self, update: PresenceUpdate) -> Result<(), GatewayError> {
        self._send(WebsocketPacket::PresenceUpdate(update)).await
    }

    /// Updates the voice status of the current shard in a certain channel.
    #[allow(dead_code)]
    pub async fn voice_state_update(
        &mut self,
        update: VoiceStateUpdate,
    ) -> Result<(), GatewayError> {
        self._send(WebsocketPacket::VoiceStateUpdate(update)).await
    }
    /// Ask discord for more informations about offline guild members.
    #[allow(dead_code)]
    pub async fn request_guild_members(
        &mut self,
        request: RequestGuildMembers,
    ) -> Result<(), GatewayError> {
        self._send(WebsocketPacket::RequestGuildMembers(request))
            .await
    }

    pub async fn _send_heartbeat(&mut self) -> Result<(), GatewayError> {
        self._send(WebsocketPacket::Heartbeat(
            self.state.as_ref().unwrap().sequence,
        ))
        .await
    }
}
