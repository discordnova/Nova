use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use super::presence::PresenceUpdate;


#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Intents {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildBans = 1 << 2,
    GuildEmojisAndStickers = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhoks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessagesReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessagesReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProprerties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}

/// Messages sent by the shard to log-in to the gateway.
#[derive(Debug, Serialize, Deserialize)]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyProprerties,
    pub compress: Option<bool>,
    pub large_threshold: Option<u64>,
    pub shard: Option<[u64; 2]>,
    pub presence: Option<PresenceUpdate>,
    pub intents: BitFlags<Intents>,
}