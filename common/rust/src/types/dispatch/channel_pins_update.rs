use serde::{Deserialize, Serialize};

/// See [docs](https://discord.com/developers/docs/topics/gateway#channel-pins-update)
#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelPinsUpdate {
    guild_id: Option<String>,
    channel_id: String,
    last_pin_timestamp: Option<String>,
}
