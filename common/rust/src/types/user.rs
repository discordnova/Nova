use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::guild::Integration;
use crate::types::utils::enumflags2_serde::from_enumflag2_truncated;

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum UserFlags {
    // None = 0 << 0,
    DiscordEmployee = 1 << 0,
    PartneredServerOwner = 1 << 1,
    HypesquadEvents = 1 << 2,
    BugHunterLevel1 = 1 << 3,
    HouseBravery = 1 << 6,
    HouseBrilliance = 1 << 7,
    HouseBalance = 1 << 8,
    EarlySupporter = 1 << 9,
    TeamUser = 1 << 10,
    BugHunterLevel2 = 1 << 14,
    VerifiedBot = 1 << 16,
    EarlyVerifiedBotDeveloper = 1 << 17,
    DiscordCertifiedModerator = 1 << 18,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PremiumTypes {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Represents a User within Discord
pub struct FullUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub premium_type: Option<PremiumTypes>,

    #[serde(deserialize_with = "from_enumflag2_truncated")]
    pub public_flags: BitFlags<UserFlags>,
    
    #[serde(deserialize_with = "from_enumflag2_truncated")]
    pub flags: BitFlags<UserFlags>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialUser {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum User {
    FullUser(FullUser),
    PartialUser(PartialUser)
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VisibilityTypes {
    None = 0,
    Everyone = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// The connection object that the user has attached.
pub struct Connection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub revoked: Option<bool>,
    pub integrations: Option<Vec<Integration>>,
    pub verified: bool,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub visibility: VisibilityTypes,
}
