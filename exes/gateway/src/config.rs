use shared::serde::{Deserialize, Serialize};
use twilight_gateway::Intents;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub token: String,
    pub intents: Intents
}

impl Default for Config {
    fn default() -> Self {
        Self { intents: Intents::empty(), token: String::default() }
    }
}
