use serde::{Deserialize, Serialize};

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload {
    pub tracing: Tracing,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CacheData {
    Ready {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadyCacheData {
    pub version: u8,
}
