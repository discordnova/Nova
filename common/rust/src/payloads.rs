use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::discord_models::{
    application::Application,
    channel::{Channel, Message, ThreadMember},
    emoji::Emoji,
    gateway::PresenceUpdate,
    guild::{Guild, GuildMember, Integration},
    invite::InviteTargetTypes,
    permissions::Role,
    slash_commands::{ApplicationCommand, Interaction},
    stage_instance::StageInstance,
    user::User,
    voice::VoiceState,
};

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload {
    pub tracing: Tracing,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CacheData {
    Ready {
        version: u8,
        user: User,
        guilds: Vec<Guild>,
        session_id: String,
        shard: Option<Vec<i64>>,
        application: Application,
    },
    ApplicationCommandCreate {
        guild_id: Option<String>,
        command: ApplicationCommand,
    },
    ApplicationCommandUpdate {
        guild_id: Option<String>,
        command: ApplicationCommand,
    },
    ApplicationCommandDelete {
        guild_id: Option<String>,
        command: ApplicationCommand,
    },
    ChannelCreate {
        channel: Channel,
    },
    ChannelUpdate {
        channel: Channel,
    },
    ChannelDelete {
        channel: Channel,
    },
    ThreadCreate {
        channel: Channel,
    },
    ThreadUpdate {
        channel: Channel,
    },
    ThreadDelete {
        channel: Channel,
    },
    ThreadListSync {
        guild_id: String,
        channel_ids: Option<Vec<String>>,
        threads: Vec<Channel>,
        members: Vec<ThreadMember>,
    },
    ThreadMemberUpdate {
        member: ThreadMember,
    },
    ThreadMembersUpdate {
        id: String,
        guild_id: String,
        member_count: i64,
        added_members: Option<Vec<ThreadMember>>,
        removed_member_ids: Option<Vec<String>>,
    },
    ChannelPinsUpdate {
        guild_id: Option<String>,
        channel_id: String,
        last_pin_timestamp: Option<String>,
    },
    GuildCreate {
        guild: Guild,
    },
    GuildUpdate {
        guild: Guild,
    },
    GuildDelete {
        guild: Guild,
    },
    GuildBanAdd {
        guild_id: String,
        user: User,
    },
    GuildBanRemove {
        guild_id: String,
        user: User,
    },
    GuildEmojisUpdate {
        guild_id: String,
        emojis: Vec<Emoji>,
    },
    GuildIntegrationsUpdate {
        guild_id: String,
    },
    GuildMemberAdd {
        guild_id: String,
        member: GuildMember,
    },
    GuildMemberRemove {
        guild_id: String,
        user: User,
    },
    GuildMemberUpdate {
        guild_id: String,
        roles: Vec<String>,
        user: User,
        nick: Option<String>,
        joined_at: Option<String>,
        premium_since: Option<String>,
        deaf: Option<bool>,
        mute: Option<bool>,
        pending: Option<bool>,
    },
    GuildMembersChunk {
        guild_id: String,
        members: Vec<GuildMember>,
        chunk_index: i64,
        chunk_count: i64,
        not_found: Option<Vec<String>>,
        presences: Option<Vec<PresenceUpdate>>,
        nonce: Option<String>,
    },
    GuildRoleCreate {
        guild_id: String,
        role: Role,
    },
    GuildRoleUpdate {
        guild_id: String,
        role: Role,
    },
    GuildRoleDelete {
        guild_id: String,
        role_id: String,
    },
    IntegrationCreate {
        guild_id: String,
        integration: Integration,
    },
    IntegrationUpdate {
        guild_id: String,
        integration: Integration,
    },
    IntegrationDelete {
        id: String,
        guild_id: String,
        application_id: Option<String>,
    },
    InviteCreate {
        channel_id: String,
        code: String,
        created_at: String,
        guild_id: Option<String>,
        inviter: Option<User>,
        max_age: i64,
        max_uses: i64,
        target_type: Option<InviteTargetTypes>,
        target_user: Option<User>,
        target_application: Option<Application>,
        temporary: bool,
        uses: i64,
    },
    InviteDelete {
        channel_id: String,
        guild_id: Option<String>,
        code: String,
    },
    InteractionCreate {
        // boxed to avoid a large difference size between variants (https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant)
        interaction: Box<Interaction>,
    },
    MessageCreate {
        message: Message,
    },
    MessageUpdate {
        message: Message,
    },
    MessageDelete {
        id: String,
        channel_id: String,
        guild_id: Option<String>,
    },
    MessageDeleteBulk {
        ids: Vec<String>,
        channel_id: String,
        guild_id: Option<String>,
    },
    MessageReactionAdd {
        user_id: String,
        channel_id: String,
        message_id: String,
        guild_id: Option<String>,
        member: Option<GuildMember>,
        emoji: Emoji,
    },
    MessageReactionRemove {
        user_id: String,
        channel_id: String,
        message_id: String,
        guild_id: Option<String>,
        emoji: Emoji,
    },
    MessageReactionRemoveAll {
        channel_id: String,
        message_id: String,
        guild_id: Option<String>,
    },
    MessageReactionRemoveEmoji {
        channel_id: String,
        message_id: String,
        guild_id: Option<String>,
        emoji: Emoji,
    },
    PresenceUpdate {
        presence: PresenceUpdate,
    },
    TypingStart {
        channel_id: String,
        guild_id: Option<String>,
        user_id: String,
        timestamp: i64,
        member: Option<GuildMember>,
    },
    UserUpdate {
        user: User,
    },
    VoiceStateUpdate {
        state: VoiceState,
    },
    VoiceServerUpdate {
        token: String,
        guild_id: String,
        endpoint: Option<String>,
    },
    WebhookUpdate {
        guild_id: String,
        channel_id: String,
    },
    StageInstanceCreate {
        instance: StageInstance,
    },
    StageInstanceUpdate {
        instance: StageInstance,
    },
    StageInstanceDelete {
        instance: StageInstance,
    },
}
