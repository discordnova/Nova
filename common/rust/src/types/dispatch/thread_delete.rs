use serde::{Deserialize, Serialize};

/// See [docs](https://discord.com/developers/docs/topics/gateway#thread-delete)
#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadDelete {
    id: String,
    guild_id: String,
    parent_id: String,
    type_: String,
}
