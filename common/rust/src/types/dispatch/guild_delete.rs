use serde::{Deserialize, Serialize};

const fn default_unavailable() -> bool {
   false
}


// todo: move to main types
#[derive(Deserialize, Serialize, Debug)]
pub struct GuildDelete {
    pub id: String,
    #[serde(default = "default_unavailable")]
    pub unavailable: bool,
}
