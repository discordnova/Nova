use serde::{Deserialize, Serialize};

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload<T> {
    pub tracing: (),
    pub data: T
}
