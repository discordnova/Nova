use std::str::from_utf8;
use tokio_tungstenite::tungstenite::Message;
use common::{log::info, types::ws::websocket::{BasePacket, WebsocketPacket}};

use crate::error::GatewayError;

use super::Connection;

impl Connection {

    /// Handles the websocket events and calls the _handle_discord_message function for the deserialization.
    pub(super) async fn _handle_message(
        &mut self,
        data: &Message,
    ) -> Result<BasePacket, GatewayError> {
        match data {
            Message::Text(text) => self._handle_discord_message(&text).await,
            Message::Binary(message) => {
                match from_utf8(message) {
                    Ok(data) => self._handle_discord_message(data).await,
                    Err(err) => Err(GatewayError::from(err.to_string())),
                }
            },
            Message::Close(close_frame) => {
                info!("Discord connection closed {:?}", close_frame);
                Err(GatewayError::from("connection closed".to_string()))
            },
            _ => Err(GatewayError::from(format!("unknown variant of message specified to the handler {}", data).to_string())),
        }
    }

    /// Handle the decompression and deserialization process of a discord payload.
    pub(super) async fn _handle_discord_message(
        &mut self,
        raw_message: &str,
    ) -> Result<BasePacket, GatewayError> {
        match serde_json::from_str(raw_message) {
            Ok(message) => Ok(message),
            Err(err) => Err(GatewayError::from(err.to_string())),
        }
    }
}
