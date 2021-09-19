use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}