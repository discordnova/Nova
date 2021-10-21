use serde::{Deserialize, Serialize};

// todo: move to main types
#[derive(Deserialize, Serialize, Debug)]
pub struct GuildDelete {
    pub id: String,
    pub unavailable: bool,
}
