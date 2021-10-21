use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageDelete {
    id: String,
    channel_id: String,
    guild_id: Option<String>,
}
