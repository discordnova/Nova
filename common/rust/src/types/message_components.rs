use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::emoji::Emoji;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ComponentTypes {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Global representation of a Component, more detailed are Button (if button) and Select Menu (if select menu)
/// Action Rows are just type: 1, content + components
pub struct Component {
    #[serde(rename = "type")]
    pub type_: ComponentTypes,
    pub custom_id: Option<String>,
    pub disabled: Option<bool>,
    pub style: Option<ButtonStyles>,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub url: Option<String>,
    pub options: Vec<SelectOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
    pub components: Option<Vec<Component>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    /// it should be `2`
    pub type_: ComponentTypes,
    pub style: ButtonStyles,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ButtonStyles {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectMenu {
    /// it should be 3
    pub type_: ComponentTypes,
    pub custom_id: String,
    pub options: Vec<SelectOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
    pub disabled: Option<bool>,
}
