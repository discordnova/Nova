use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    application::Application, emoji::Emoji, guild::GuildMember, message_components::Component,
    slash_commands::MessageInteraction, user::User,
};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ChannelTypes {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VideoQualityModes {
    Auto = 1,
    Full = 2,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum OverwriteTypes {
    Role = 0,
    Member = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Overwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: OverwriteTypes,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i64,
    pub archive_timestamp: String,
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ChannelTypes,
    pub guild_id: Option<String>,
    pub position: Option<i64>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i64>,
    pub user_limit: Option<i64>,
    pub rate_limit_per_user: Option<i64>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityModes>,
    pub message_count: Option<i64>,
    pub member_count: Option<i64>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,
    #[serde(rename = "type")]
    pub type_: ChannelTypes,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EmbedTypes {
    #[serde(rename = "rich")]
    Rich,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "gifv")]
    Gifv,
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "link")]
    Link,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedThumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedImage {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<EmbedTypes>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i64>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reaction {
    pub count: i64,
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MessageTypes {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 20,
    ApplicationCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MessageActivityTypes {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub type_: MessageActivityTypes,
    pub party_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReference {
    pub message_id: Option<String>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub fail_if_not_exists: Option<bool>,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum MessageFlags {
    Crossposted = 1 << 0,
    IsCrosspost = 1 << 1,
    SuppressEmbeds = 1 << 2,
    SourceMessageDeleted = 1 << 3,
    Urgent = 1 << 4,
    HasThread = 1 << 5,
    Ephemeral = 1 << 6,
    Loading = 1 << 7,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MessageStickerFormatTypes {
    Png = 1,
    Apng = 2,
    Lottie = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageStickerItem {
    pub id: String,
    pub name: String,
    pub format_type: MessageStickerFormatTypes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: String,
    pub tags: String,
    // deprecated
    // pub asset: String,
    pub format_type: MessageStickerFormatTypes,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<User>,
    pub sort_value: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub author: User,
    pub member: Option<GuildMember>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: String,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>, // todo: It is a Vector of User objects, with an additional partial member field (GuildMember)
    pub mentions_roles: Vec<String>,
    pub mention_channels: Option<Vec<Channel>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    /// ! Can be a String or an int !
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: MessageTypes,
    pub activity: Option<MessageActivity>,
    pub application: Option<Application>,
    pub application_id: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<BitFlags<MessageFlags>>,
    pub referenced_message: Option<Message>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Component>,
    pub sticker_items: Option<Vec<MessageStickerItem>>,
    // deprecated
    // pub stickers: Option<Vec<MessageSticker>>,
}
