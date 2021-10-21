use crate::types::{application::Application, invite::InviteTargetTypes, user::User};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteCreate {
    pub channel_id: String,
    pub code: String,
    pub created_at: String,
    pub guild_id: Option<String>,
    pub inviter: Option<User>,
    pub max_age: i64,
    pub max_uses: i64,
    pub target_type: Option<InviteTargetTypes>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub temporary: bool,
    pub uses: i64,
}