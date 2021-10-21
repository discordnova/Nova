use serde::{Deserialize, Serialize};
use crate::types::{application::Application, guild::Guild, user::User};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ready {
    #[serde(rename = "v")]
    pub version: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub session_id: String,
    pub shard: Option<Vec<i64>>,
    pub application: Application,
}
