use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveAll {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
}