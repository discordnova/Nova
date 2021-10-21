use crate::types::emoji::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveEmoji {
    channel_id: String,
    guild_id: Option<String>,
    message_id: String,
    emoji: Emoji,
}