use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleTags {
  pub bot_id: Option<String>,
  pub integration_id: Option<String>,
  pub premium_subscriber: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role {
  pub id: String,
  pub name: String,
  pub color: i64,
  pub hoist: bool,
  pub position: i64,
  pub permissions: String,
  pub managed: bool,
  pub mentionable: bool,
  pub tags: Option<RoleTags>,
}
