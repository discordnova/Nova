use serde::Deserialize;

/// This struct represents a message sent by the discord gateway
/// 
pub struct WebsocketMessage {}

impl Deserialize<'de> for WebsocketMessage {
    
}