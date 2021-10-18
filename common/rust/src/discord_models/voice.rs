use serde::{Deserialize, Serialize};

use super::guild::GuildMember;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_id: String,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    pub vip: bool,
    pub optimal: bool,
    pub deprecated: bool,
    pub custom: bool,
}
