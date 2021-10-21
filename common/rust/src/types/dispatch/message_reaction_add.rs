use crate::types::{emoji::Emoji, guild::GuildMember};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageReactionAdd {
    user_id: String,
    message_id: String,
    channel_id: String,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    emoji: Emoji,
}
