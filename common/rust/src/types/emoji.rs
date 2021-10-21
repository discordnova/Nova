use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub roles: Vec<String>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
