use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteDelete {
    channel_id: String,
    guild_id: Option<String>,
    code: String,
}
