use crate::types::guild::GuildMember;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TypingStart {
    channel_id: String,
    guild_id: Option<String>,
    user_id: String,
    timestamp: i64,
    member: Option<GuildMember>,
}