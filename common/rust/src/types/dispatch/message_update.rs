use serde::{Deserialize, Serialize};

// todo: move as partial message
#[derive(Deserialize, Debug, Serialize)]
pub struct MessageUpdate {
    id: String,
    channel_id: String,
}