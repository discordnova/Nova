use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use twilight_model::gateway::event::Event;
use crate::serializable_event::SerializableEvent;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SerializeHelper(#[serde(with = "SerializableEvent")] pub Event);

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload {
    pub tracing: Tracing,
    #[serde(flatten)]
    pub data: SerializeHelper,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}
