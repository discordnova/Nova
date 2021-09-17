use super::Connection;
use crate::client::{error_utils::GatewayError};
use std::str::from_utf8;
use tokio_tungstenite::tungstenite::Message;

impl Connection {
    pub(crate) async fn _handle_message(
        &mut self,
        data: &Message,
    ) -> Result<crate::client::payloads::gateway::Message, GatewayError> {
        match data {
            Message::Text(text) => self._handle_discord_message(&text).await,
            Message::Binary(message) => {
                self._handle_discord_message(from_utf8(message).unwrap())
                    .await
            }
            _ => Err(GatewayError::from("unknown error".to_string())),
        }
    }

    async fn _handle_discord_message(
        &mut self,
        raw_message: &str,
    ) -> Result<crate::client::payloads::gateway::Message, GatewayError> {
        let a: Result<crate::client::payloads::gateway::Message, serde_json::Error> = serde_json::from_str(raw_message);
        let message = a.unwrap();
        Ok(message)
    }
}
