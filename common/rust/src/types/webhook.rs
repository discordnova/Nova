use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{channel::Channel, guild::Guild, user::User};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum WebhookTypes {
    Incoming = 1,
    ChannelFollower = 2,
    Application = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: WebhookTypes,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub token: Option<String>,
    pub application_id: Option<String>,
    pub source_guild: Option<Guild>,
    pub source_channel: Option<Channel>,
    pub url: String,
}
