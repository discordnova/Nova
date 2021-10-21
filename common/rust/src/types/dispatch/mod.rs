use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Serialize};

use self::voice_server_update::VoiceServerUpdate;
use self::{
    channel_create::ChannelCreate, channel_delete::ChannelDelete,
    channel_pins_update::ChannelPinsUpdate, channel_update::ChannelUpdate,
    guild_ban_add::GuildBanAdd, guild_ban_remove::GuildBanRemove, guild_create::GuildCreate,
    guild_delete::GuildDelete, guild_emojis_update::GuildEmojisUpdate,
    guild_integrations_update::GuildIntegrationsUpdate, guild_member_add::GuildMemberAdd,
    guild_member_remove::GuildMemberRemove, guild_member_update::GuildMemberUpdate,
    guild_members_chunk::GuildMembersChunk, guild_role_create::GuildRoleCreate,
    guild_role_delete::GuildRoleDelete, guild_role_update::GuildRoleUpdate,
    guild_stickers_update::GuildStickersUpdate, guild_update::GuildUpdate,
    integration_create::IntegrationCreate, integration_delete::IntegrationDelete,
    integration_update::IntegrationUpdate, interaction_create::InteractionCreate,
    invite_create::InviteCreate, invite_delete::InviteDelete, message_create::MessageCreate,
    message_delete::MessageDelete, message_delete_bulk::MessageDeleteBulk,
    message_reaction_add::MessageReactionAdd, message_reaction_remove::MessageReactionRemove,
    message_reaction_remove_all::MessageReactionRemoveAll,
    message_reaction_remove_emoji::MessageReactionRemoveEmoji, message_update::MessageUpdate,
    presence_update::PresenceUpdate, ready::Ready, resumed::Resumed,
    stage_instance_create::StageInstanceCreate, stage_instance_delete::StageInstanceDelete,
    stage_instance_update::StageInstanceUpdate, thread_create::ThreadCreate,
    thread_delete::ThreadDelete, thread_list_sync::ThreadListSync,
    thread_member_update::ThreadMemberUpdate, thread_members_update::ThreadMembersUpdate,
    thread_update::ThreadUpdate, typing_start::TypingStart, user_update::UserUpdate,
    voice_state_update::VoiceStateUpdate, webhook_update::WebhookUpdate,
};
use crate::types::ws::websocket::BasePacket;

pub mod application_command_create;
pub mod application_command_delete;
pub mod application_command_update;
pub mod channel_create;
pub mod channel_delete;
pub mod channel_pins_update;
pub mod channel_update;
pub mod guild_ban_add;
pub mod guild_ban_remove;
pub mod guild_create;
pub mod guild_delete;
pub mod guild_emojis_update;
pub mod guild_integrations_update;
pub mod guild_member_add;
pub mod guild_member_remove;
pub mod guild_member_update;
pub mod guild_members_chunk;
pub mod guild_role_create;
pub mod guild_role_delete;
pub mod guild_role_update;
pub mod guild_stickers_update;
pub mod guild_update;
pub mod integration_create;
pub mod integration_delete;
pub mod integration_update;
pub mod interaction_create;
pub mod invite_create;
pub mod invite_delete;
pub mod message_create;
pub mod message_delete;
pub mod message_delete_bulk;
pub mod message_reaction_add;
pub mod message_reaction_remove;
pub mod message_reaction_remove_all;
pub mod message_reaction_remove_emoji;
pub mod message_update;
pub mod presence_update;
pub mod ready;
pub mod resumed;
pub mod stage_instance_create;
pub mod stage_instance_delete;
pub mod stage_instance_update;
pub mod thread_create;
pub mod thread_delete;
pub mod thread_list_sync;
pub mod thread_member_update;
pub mod thread_members_update;
pub mod thread_update;
pub mod typing_start;
pub mod user_update;
pub mod voice_server_update;
pub mod voice_state_update;
pub mod webhook_update;
use paste::paste;

macro_rules! generate_enums {
    ($($name: tt: $type: tt,)+) => {

        #[derive(Deserialize, Serialize, Debug)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum DispatchType {
            $(
                $name,
            )+
        }

        #[derive(Serialize, Debug, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[serde(tag = "type")]
        pub enum Dispatch {
            $($name($type),)+
        }

        impl From<BasePacket> for Result<Dispatch, serde_json::Error> {
            fn from(packet: BasePacket) -> Self {
                match packet.type_.unwrap() {
                    $(DispatchType::$name => serde_json::from_value(packet.data.unwrap()).map(Dispatch::$name)
                    .map_err(SerdeDeError::custom),)+
                }
            }
        }

        paste! {
            impl Dispatch {
                pub fn snake_case_name(&self) -> String {
                    match self {
                        $(Dispatch::$name(_) => stringify!([<$name:snake>]).to_string(),)+
                    }
                }
            }
        }
    }
}

generate_enums!(
    Ready: Ready,
    Resumed: Resumed,
    ChannelCreate: ChannelCreate,
    ChannelUpdate: ChannelUpdate,
    ChannelDelete: ChannelDelete,
    ChannelPinsUpdate: ChannelPinsUpdate,
    ThreadCreate: ThreadCreate,
    ThreadUpdate: ThreadUpdate,
    ThreadDelete: ThreadDelete,
    ThreadListSync: ThreadListSync,
    ThreadMemberUpdate: ThreadMemberUpdate,
    ThreadMembersUpdate: ThreadMembersUpdate,
    GuildCreate: GuildCreate,
    GuildUpdate: GuildUpdate,
    GuildDelete: GuildDelete,
    GuildBanAdd: GuildBanAdd,
    GuildBanRemove: GuildBanRemove,
    GuildEmojisUpdate: GuildEmojisUpdate,
    GuildStickersUpdate: GuildStickersUpdate,
    GuildIntegrationsUpdate: GuildIntegrationsUpdate,
    GuildMemberAdd: GuildMemberAdd,
    GuildMemberRemove: GuildMemberRemove,
    GuildMemberUpdate: GuildMemberUpdate,
    GuildMembersChunk: GuildMembersChunk,
    GuildRoleCreate: GuildRoleCreate,
    GuildRoleUpdate: GuildRoleUpdate,
    GuildRoleDelete: GuildRoleDelete,
    IntegrationCreate: IntegrationCreate,
    IntegrationUpdate: IntegrationUpdate,
    IntegrationDelete: IntegrationDelete,
    InteractionCreate: InteractionCreate,
    InviteCreate: InviteCreate,
    InviteDelete: InviteDelete,
    MessageCreate: MessageCreate,
    MessageUpdate: MessageUpdate,
    MessageDelete: MessageDelete,
    MessageDeleteBulk: MessageDeleteBulk,
    MessageReactionAdd: MessageReactionAdd,
    MessageReactionRemove: MessageReactionRemove,
    MessageReactionRemoveAll: MessageReactionRemoveAll,
    MessageReactionRemoveEmoji: MessageReactionRemoveEmoji,
    PresenceUpdate: PresenceUpdate,
    StageInstanceCreate: StageInstanceCreate,
    StageInstanceDelete: StageInstanceDelete,
    StageInstanceUpdate: StageInstanceUpdate,
    TypingStart: TypingStart,
    UserUpdate: UserUpdate,
    VoiceStateUpdate: VoiceStateUpdate,
    VoiceServerUpdate: VoiceServerUpdate,
    WebhooksUpdate: WebhookUpdate,
);
