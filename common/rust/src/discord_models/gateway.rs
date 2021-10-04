use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PresenceUpdateStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresenceUpdate {
    pub user: User,
    pub guild_id: String,
    pub status: PresenceUpdateStatus,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ActivityTypes {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityTimestamps {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityParty {
    pub id: Option<String>,
    /// [current_size, max_size]
    pub size: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub match_: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityButtons {
    pub label: String,
    pub url: String,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum ActivityFlags {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinRequest = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ActivityTypes,
    pub url: Option<String>,
    pub created_at: String,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<BitFlags<ActivityFlags>>,
    pub buttons: Option<Vec<ActivityButtons>>,
}
