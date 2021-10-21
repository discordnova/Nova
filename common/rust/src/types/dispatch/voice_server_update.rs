use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VoiceServerUpdate {
    token: String,
    guild_id: String,
    endpoint: Option<String>,
}
