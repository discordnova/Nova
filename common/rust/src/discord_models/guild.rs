use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    channel::Channel, emoji::Emoji, gateway::PresenceUpdate, permissions::Role,
    stage_instance::StageInstance, user::User, voice::VoiceState,
};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntegrationApplication {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub summary: Option<String>,
    pub bot: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Integration {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    pub expire_grace_period: Option<i64>,
    pub user: Option<User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<String>,
    pub subscriber_count: Option<i64>,
    pub revoked: Option<bool>,
    pub application: Option<IntegrationApplication>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MfaLevel {
    None = 0,
    Elevated = 1,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum GuildNsfwLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuildFeatures {
    #[serde(rename = "ANIMATED_ICON")]
    AnimatedIcon,
    #[serde(rename = "BANNER")]
    Banner,
    #[serde(rename = "COMMERCE")]
    Commerce,
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "DISCOVERABLE")]
    Discoverable,
    #[serde(rename = "FEATURABLE")]
    Featurable,
    #[serde(rename = "INVITE_SPLASH")]
    InviteSplash,
    #[serde(rename = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled,
    #[serde(rename = "NEWS")]
    News,
    #[serde(rename = "PARTNERED")]
    Partnered,
    #[serde(rename = "PREVIEW_ENABLED")]
    PreviewEnabled,
    #[serde(rename = "VANITY_URL")]
    VanityUrl,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "VIP_REGIONS")]
    VipRegions,
    #[serde(rename = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled,
    #[serde(rename = "TICKETED_EVENTS_ENABLED")]
    TicketedEventsEnabled,
    #[serde(rename = "MONETIZATION_ENABLED")]
    MonetizationEnabled,
    #[serde(rename = "MORE_STICKERS")]
    MoreStickers,
    #[serde(rename = "THREE_DAY_THREAD_ARCHIVE")]
    ThreeDayThreadArchive,
    #[serde(rename = "SEVEN_DAY_THREAD_ARCHIVE")]
    SevenDayThreadArchive,
    #[serde(rename = "PRIVATE_THREADS")]
    PrivateThreads,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum SystemChannelFlags {
    SuppressJoinNotifications = 1 << 0,
    SuppressPremiumSubscriptions = 1 << 1,
    SuppressGuildReminderNotifications = 1 << 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WelcomeScreen {
    pub description: String,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<String>,
    /// DEPRECATED
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: i64,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilter,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<GuildFeatures>,
    pub mfa_level: MfaLevel,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: BitFlags<SystemChannelFlags>,
    pub rules_channel_id: Option<String>,
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<i64>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub members: Option<Vec<GuildMember>>,
    pub channels: Option<Vec<Channel>>,
    pub threads: Option<Vec<Channel>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub max_presences: Option<i64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: i64,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<i64>,
    pub approximate_member_count: Option<i64>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: GuildNsfwLevel,
    pub stage_instances: Option<Vec<StageInstance>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildPreview {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<GuildFeatures>,
    pub approximate_member_count: i64,
    pub approximate_presence_count: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildWidget {
    pub enabled: bool,
    pub channel_id: Option<String>,
}
