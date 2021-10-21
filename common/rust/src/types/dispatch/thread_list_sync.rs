use crate::types::channel::{Channel, ThreadMember};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadListSync {
    guild_id: String,
    channel_ids: Option<Vec<String>>,
    threads: Vec<Channel>,
    members: Vec<ThreadMember>,
}