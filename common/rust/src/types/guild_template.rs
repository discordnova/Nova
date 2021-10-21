use serde::{Deserialize, Serialize};

use super::{guild::Guild, user::User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildTemplate {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub usage_count: i64,
    pub creator_id: String,
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub source_guild_ild: String,
    pub serialized_source_guild: Guild,
    pub is_dirty: Option<bool>,
}
