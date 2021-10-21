use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::user::User;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MembershipState {
    Invited = 0,
    Accepted = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamMembers {
    pub membership_state: MembershipState,
    pub permissions: Vec<String>,
    pub team_id: String,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Team {
    pub icon: Option<String>,
    pub id: String,
    pub members: Vec<TeamMembers>,
    pub name: String,
    pub owner_user_id: String,
}
