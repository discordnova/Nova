use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    application::Application,
    channel::Channel,
    guild::{Guild, GuildMember},
    user::User,
};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum InviteTargetTypes {
    Stream = 1,
    EmbeddedApplication = 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteStageInstance {
    pub members: Vec<GuildMember>,
    pub participant_count: i64,
    pub speaker_count: i64,
    pub topic: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invite {
    pub code: String,
    pub guild: Option<Guild>,
    pub channel: Channel,
    pub inviter: Option<User>,
    pub target_type: Option<InviteTargetTypes>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub approximate_presence_count: Option<i64>,
    pub approximate_member_count: Option<i64>,
    pub expires_at: Option<String>,
    pub stage_instance: Option<InviteStageInstance>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteMetadata {
    pub uses: i64,
    pub max_uses: i64,
    pub max_age: i64,
    pub temporary: bool,
    pub created_at: String,
}
