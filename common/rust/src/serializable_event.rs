use serde::{Deserialize, Serialize};
use twilight_model::gateway::{
    event::{
        shard::{
            Connected, Connecting, Disconnected, Identifying, Payload, Reconnecting, Resuming,
        },
        Event,
    },
    payload::{
        BanAdd, BanRemove, ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate,
        GuildCreate, GuildDelete, GuildEmojisUpdate, GuildIntegrationsUpdate, GuildUpdate,
        IntegrationCreate, IntegrationDelete, IntegrationUpdate, InteractionCreate, InviteCreate,
        InviteDelete, MemberAdd, MemberChunk, MemberRemove, MemberUpdate, MessageCreate,
        MessageDelete, MessageDeleteBulk, MessageUpdate, PresenceUpdate, ReactionAdd,
        ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji, Ready, RoleCreate, RoleDelete,
        RoleUpdate, StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate, ThreadCreate,
        ThreadDelete, ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, ThreadUpdate,
        TypingStart, UnavailableGuild, UserUpdate, VoiceServerUpdate, VoiceStateUpdate,
        WebhooksUpdate,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Event")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "t", content = "c")]
pub enum SerializableEvent {
    /// A user was banned from a guild.
    BanAdd(BanAdd),
    /// A user's ban from a guild was removed.
    BanRemove(BanRemove),
    /// A channel was created.
    ChannelCreate(ChannelCreate),
    /// A channel was deleted.
    ChannelDelete(ChannelDelete),
    /// A channel's pins were updated.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// A channel was updated.
    ChannelUpdate(ChannelUpdate),
    /// A heartbeat was sent to or received from the gateway.
    GatewayHeartbeat(u64),
    /// A heartbeat acknowledgement was received from the gateway.
    GatewayHeartbeatAck,
    /// A "hello" packet was received from the gateway.
    GatewayHello(u64),
    /// A shard's session was invalidated.
    ///
    /// `true` if resumeable. If not, then the shard must do a full reconnect.
    GatewayInvalidateSession(bool),
    /// The gateway is indicating to perform a reconnect.
    GatewayReconnect,
    /// Undocumented event, should be ignored
    GiftCodeUpdate,
    /// A guild was created.
    GuildCreate(Box<GuildCreate>),
    /// A guild was deleted or the current user was removed from a guild.
    GuildDelete(Box<GuildDelete>),
    /// A guild's emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// A guild's integrations were updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// A guild was updated.
    GuildUpdate(Box<GuildUpdate>),
    /// A guild integration was created.
    IntegrationCreate(Box<IntegrationCreate>),
    /// A guild integration was updated.
    IntegrationDelete(IntegrationDelete),
    /// A guild integration was deleted.
    IntegrationUpdate(Box<IntegrationUpdate>),
    /// An interaction was invoked by a user.
    InteractionCreate(Box<InteractionCreate>),
    /// A invite was made.
    InviteCreate(Box<InviteCreate>),
    /// A invite was deleted.
    InviteDelete(InviteDelete),
    /// A user was added to a guild.
    MemberAdd(Box<MemberAdd>),
    /// A user was removed from a guild.
    MemberRemove(MemberRemove),
    /// A user's member object in a guild was updated.
    MemberUpdate(Box<MemberUpdate>),
    /// A chunk of members were received from the gateway.
    MemberChunk(MemberChunk),
    /// A message was created in a channel.
    MessageCreate(Box<MessageCreate>),
    /// A message was deleted in a channel.
    MessageDelete(MessageDelete),
    /// Multiple messages were deleted in a channel.
    MessageDeleteBulk(MessageDeleteBulk),
    /// A message was updated in a channel.
    MessageUpdate(Box<MessageUpdate>),
    /// A user's active presence (such as game or online status) was updated.
    PresenceUpdate(Box<PresenceUpdate>),
    /// Multiple presences outside of a guild were updated.
    ///
    /// For bots this is always empty and useless.
    PresencesReplace,
    /// A reaction was added to a message.
    ReactionAdd(Box<ReactionAdd>),
    /// A reaction was removed from a message.
    ReactionRemove(Box<ReactionRemove>),
    /// All reactions were removed from a message.
    ReactionRemoveAll(ReactionRemoveAll),
    /// All instances of a given emoji from the reactions of a message were
    /// removed.
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    /// A shard is now "ready" and fully connected.
    Ready(Box<Ready>),
    /// A shard has successfully resumed.
    Resumed,
    /// A role was created in a guild.
    RoleCreate(RoleCreate),
    /// A role was deleted in a guild.
    RoleDelete(RoleDelete),
    /// A role was updated in a guild.
    RoleUpdate(RoleUpdate),
    /// A shard is now in a connected stage after being fully connected to the
    /// gateway.
    ShardConnected(Connected),
    /// A shard is now in a connecting stage after starting to connect to the
    /// gateway.
    ShardConnecting(Connecting),
    /// A shard is now in a disconnected stage after the connection was closed.
    ShardDisconnected(Disconnected),
    /// A shard is now in a identifying stage after starting a new session.
    ShardIdentifying(Identifying),
    /// A shard is now in a reconnecting stage after a disconnect or session was
    /// ended.
    ShardReconnecting(Reconnecting),
    /// A payload of bytes came in through the shard's connection.
    ShardPayload(Payload),
    /// A shard is now in a Resuming stage after a disconnect.
    ShardResuming(Resuming),
    /// A stage instance was created in a stage channel.
    StageInstanceCreate(StageInstanceCreate),
    /// A stage instance was deleted in a stage channel.
    StageInstanceDelete(StageInstanceDelete),
    /// A stage instance was updated in a stage channel.
    StageInstanceUpdate(StageInstanceUpdate),
    /// A thread has been created, relevant to the current user,
    /// or the current user has been added to a thread.
    ThreadCreate(ThreadCreate),
    /// A thread, relevant to the current user, has been deleted.
    ThreadDelete(ThreadDelete),
    /// The current user has gained access to a thread.
    ThreadListSync(ThreadListSync),
    /// The thread member object for the current user has been updated.
    ThreadMemberUpdate(ThreadMemberUpdate),
    /// A user has been added to or removed from a thread.
    ThreadMembersUpdate(ThreadMembersUpdate),
    /// A thread has been updated.
    ThreadUpdate(ThreadUpdate),
    /// A user started typing in a channel.
    TypingStart(Box<TypingStart>),
    /// A guild is now unavailable.
    UnavailableGuild(UnavailableGuild),
    /// The current user was updated.
    UserUpdate(UserUpdate),
    /// A voice server update was sent.
    VoiceServerUpdate(VoiceServerUpdate),
    /// A voice state in a voice channel was updated.
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    /// A webhook was updated.
    WebhooksUpdate(WebhooksUpdate),
}
