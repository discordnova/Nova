use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildRoleDelete {
    guild_id: String,
    role_id: String,
}