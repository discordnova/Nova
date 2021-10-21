use crate::types::permissions::Role;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildRoleCreate {
    guild_id: String,
    role: Role,
}
