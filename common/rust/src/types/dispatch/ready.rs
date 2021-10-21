use serde::{Deserialize, Serialize};
use crate::types::{application::Application, guild::Guild, user::User};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ready {
    #[serde(rename = "v")]
    version: u8,
    user: User,
    guilds: Vec<Guild>,
    session_id: String,
    shard: Option<Vec<i64>>,
    application: Application,
}
