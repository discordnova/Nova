use serde_repr::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    start: u64,
    end: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityEmoji {
    name: String,
    id: Option<String>,
    animated: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    t: ActivityType,

    url: Option<String>,
    created_at: i64,
    timestamp: Option<ActivityTimestamps>,
    application_id: Option<String>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<ActivityEmoji>,
    // todo: implement more
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PresenceStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "offline")]
    Offline
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceUpdate {
    since: u64,
    activities: Vec<Activity>,
    status: PresenceStatus,
    afk: bool,
}
