use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    /// todo: Can also be a String!!!!
    pub value: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub type_: ApplicationCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationCommand {
    pub id: String,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub name: String,
    pub description: String,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub default_permission: Option<bool>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildApplicationCommandPermissions {
    pub id: String,
    pub application_id: String,
    pub guild_id: String,
    pub permissions: Vec<ApplicationCommandPermissions>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationCommandPermissions {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: ApplicationCommandPermissionType,
    pub permission: bool,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum InteractionRequestType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: InteractionRequestType,
    pub name: String,
    pub user: User,
}
