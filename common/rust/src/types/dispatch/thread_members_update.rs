use crate::types::channel::ThreadMember;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadMembersUpdate {
    id: String,
    guild_id: String,
    member_count: i64,
    added_members: Option<Vec<ThreadMember>>,
    removed_member_ids: Option<Vec<String>>,
}