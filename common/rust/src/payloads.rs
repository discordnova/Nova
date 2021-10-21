use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use crate::types::dispatch::Dispatch;

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug)]
// #[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload {
    pub tracing: Tracing,
    pub data: Dispatch,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}
