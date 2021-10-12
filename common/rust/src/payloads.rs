use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Data structure sent to the cache component
/// by the gateway & webhook.
#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "T: Deserialize<'de> + Debug"))]
pub struct CachePayload<T> {

    #[serde(rename = "tr")]
    pub tracing: Tracing,

    #[serde(rename = "d")]
    pub data: T,

    #[serde(rename = "o")]
    pub operation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>
}
