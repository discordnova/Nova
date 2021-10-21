use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildMemberRemove {
    guild_id: String,
    user: User,
}