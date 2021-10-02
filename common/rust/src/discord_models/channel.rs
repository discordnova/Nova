use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ChannelTypes {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VideoQualityModes {
    Auto = 1,
    Full = 2,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum OverwriteTypes {
    Role = 0,
    Member = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Overwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: OverwriteTypes,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i64,
    pub archive_timestamp: String,
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ChannelTypes,
    pub guild_id: Option<String>,
    pub position: Option<i64>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i64>,
    pub user_limit: Option<i64>,
    pub rate_limit_per_user: Option<i64>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityModes>,
    pub message_count: Option<i64>,
    pub member_count: Option<i64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i64>,
}
