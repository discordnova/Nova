use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::types::channel::Channel;

use super::{guild::Integration, user::User, webhook::Webhook};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum AuditLogEvents {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OptionalAuditEntryInfoType {
    #[serde(rename = "0")]
    Role,
    #[serde(rename = "1")]
    Member,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionalAuditEntryInfo {
    pub delete_member_days: Option<String>,
    pub members_removed: Option<String>,
    pub channel_id: Option<String>,
    pub message_id: Option<String>,
    pub count: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<Box<OptionalAuditEntryInfo>>,
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuditLogChangeKey {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "icon_hash")]
    IconHash,
    #[serde(rename = "splash_hash")]
    SplashHash,
    #[serde(rename = "discovery_splash_hash")]
    DiscoverySplashHash,
    #[serde(rename = "banner_hash")]
    BannerHash,
    #[serde(rename = "owner_id")]
    OwnerId,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "preferred_locale")]
    PreferredLocale,
    #[serde(rename = "afk_channel_id")]
    AfkChannelId,
    #[serde(rename = "afk_timeout")]
    AfkTimeout,
    #[serde(rename = "rules_channel_id")]
    RulesChannelId,
    #[serde(rename = "public_updates_channel_id")]
    PublicUpdatesChannelId,
    #[serde(rename = "mfa_level")]
    MfaLevel,
    #[serde(rename = "verification_level")]
    VerificationLevel,
    #[serde(rename = "explicit_content_filter")]
    ExplicitContentFilter,
    #[serde(rename = "default_message_notifications")]
    DefaultMessageNotifications,
    #[serde(rename = "vanity_url_code")]
    VanityUrlCode,
    #[serde(rename = "$add")]
    Add,
    #[serde(rename = "$remove")]
    Remove,
    #[serde(rename = "prune_delete_days")]
    PruneDeleteDays,
    #[serde(rename = "widget_enabled")]
    WidgetEnabled,
    #[serde(rename = "widget_channel_id")]
    WidgetChannelId,
    #[serde(rename = "system_channel_id")]
    SystemChannelId,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "bitrate")]
    Bitrate,
    #[serde(rename = "permission_overwrites")]
    PermissionOverwrites,
    #[serde(rename = "nsfw")]
    Nsfw,
    #[serde(rename = "application_id")]
    ApplicationId,
    #[serde(rename = "rate_limit_per_user")]
    RateLimitPerUser,
    #[serde(rename = "permissions")]
    Permissions,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "hoist")]
    Hoist,
    #[serde(rename = "mentionable")]
    Mentionable,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "channel_id")]
    ChannelId,
    #[serde(rename = "inviter_id")]
    InviterId,
    #[serde(rename = "max_uses")]
    MaxUses,
    #[serde(rename = "uses")]
    Uses,
    #[serde(rename = "max_age")]
    MaxAge,
    #[serde(rename = "temporary")]
    Temporary,
    #[serde(rename = "deaf")]
    Deaf,
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "nick")]
    Nick,
    #[serde(rename = "avatar_hash")]
    AvatarHash,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "enable_emoticons")]
    EnableEmoticons,
    #[serde(rename = "expire_behavior")]
    ExpireBehavior,
    #[serde(rename = "expire_grace_period")]
    ExpireGracePeriod,
    #[serde(rename = "user_limit")]
    UserLimit,
    #[serde(rename = "privacy_level")]
    PrivacyLevel,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogChange {
    pub new_value: Option<String>,
    pub old_value: Option<String>,
    pub key: AuditLogChangeKey,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Option<String>,
    pub id: String,
    pub action_type: AuditLogEvents,
    pub options: Option<OptionalAuditEntryInfo>,
    pub reason: Option<String>,
}

pub enum NewAuditLogEntry {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLog {
    pub webhooks: Vec<Webhook>,
    pub users: Vec<User>,
    pub threads: Vec<Channel>,
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub integrations: Vec<Integration>,
}
