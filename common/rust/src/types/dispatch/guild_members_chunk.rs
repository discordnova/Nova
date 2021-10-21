use crate::types::guild::GuildMember;
use serde::{Deserialize, Serialize};
use super::presence_update::PresenceUpdate;

#[derive(Serialize, Debug, Deserialize)]
pub struct GuildMembersChunk {
    guild_id: String,
    members: Vec<GuildMember>,
    chunk_index: i64,
    chunk_count: i64,
    not_found: Option<Vec<String>>,
    presences: Option<Vec<PresenceUpdate>>,
    nonce: Option<String>,
}