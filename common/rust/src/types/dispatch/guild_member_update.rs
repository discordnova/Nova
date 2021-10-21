use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildMemberUpdate {
    guild_id: String,
    roles: Vec<String>,
    user: User,
    nick: Option<String>,
    avatar: Option<String>,
    joined_at: Option<String>,
    premium_since: Option<String>,
    deaf: Option<bool>,
    mute: Option<bool>,
    pending: Option<bool>,
}
