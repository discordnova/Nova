use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct WebhookUpdate {
    guild_id: String,
    channel_id: String,
}