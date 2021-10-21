use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildIntegrationsUpdate {
    guild_id: String,
}