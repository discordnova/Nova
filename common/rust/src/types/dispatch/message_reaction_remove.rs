use crate::types::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReactionRemove {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}