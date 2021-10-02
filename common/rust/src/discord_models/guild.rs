use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationApplication {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub summary: Option<String>,
    pub bot: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Integration {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    pub expire_grace_period: Option<i64>,
    pub user: Option<User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<String>,
    pub subscriber_count: Option<i64>,
    pub revoked: Option<bool>,
    pub application: Option<IntegrationApplication>,
}
