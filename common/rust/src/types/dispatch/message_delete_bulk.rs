use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDeleteBulk {
    ids: Vec<String>,
    channel_id: String,
    guild_id: Option<String>,
}