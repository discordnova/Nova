use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct IntegrationDelete {
    id: String,
    guild_id: String,
    application_id: Option<String>,
}
