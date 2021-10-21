use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use crate::types::ws::websocket::WebsocketPacket;

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug)]
pub struct CachePayload {
    pub tracing: Tracing,
    pub data: WebsocketPacket,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}
