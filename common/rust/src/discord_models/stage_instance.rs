use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PrivacyLevel {
    Public = 1,
    GuildOnly = 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StageInstance {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub topic: String,
    pub privacy_level: PrivacyLevel,
    pub discoverable_disabled: bool,
}
