use crate::types::channel::MessageSticker;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GuildStickersUpdate {
    pub guild_id: String,
    pub stickers: Vec<MessageSticker>,
}
