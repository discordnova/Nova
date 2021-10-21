use crate::types::emoji::Emoji;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildEmojisUpdate {
    guild_id: String,
    emojis: Vec<Emoji>,
}