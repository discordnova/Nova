use enumflags2::{bitflags, BitFlags};

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

pub struct Sharding {
    pub total_shards: i64,
    pub current_shard: i64
}

/// Config for the client connection.
pub struct ClientConfig {
    pub token: String,
    pub large_threshold: Option<u64>,
    pub shard: Option<Sharding>,
    pub intents: BitFlags<Intents>
}