use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildBanAdd {
    guild_id: String,
    user: User,
}