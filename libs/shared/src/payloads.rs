use std::fmt::Debug;

use crate::serde::Deserializer;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use twilight_model::gateway::event::{DispatchEvent, DispatchEventWithTypeDeserializer};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct DispatchEventTagged {
    pub data: DispatchEvent,
}

#[derive(Serialize, Deserialize)]
struct DispatchEventTaggedSerialized {
    #[serde(rename = "d")]
    pub data: Value,
    #[serde(rename = "t")]
    pub kind: String,
}

impl<'de> Deserialize<'de> for DispatchEventTagged {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tagged = DispatchEventTaggedSerialized::deserialize(deserializer)?;
        let deserializer_seed = DispatchEventWithTypeDeserializer::new(&tagged.kind);
        let dispatch_event = deserializer_seed.deserialize(tagged.data).unwrap();
        Ok(DispatchEventTagged { data: dispatch_event })
    }
}

impl Serialize for DispatchEventTagged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = self.data.kind().name().unwrap().to_string();

        let s = DispatchEventTaggedSerialized {
            kind,
            data: serde_json::to_value(&self.data).unwrap(),
        };

        s.serialize(serializer)
    }
}

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct CachePayload {
    pub tracing: Tracing,
    #[serde(flatten)]
    pub data: DispatchEventTagged,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracing {
    pub node_id: String,
    pub span: Option<String>,
}
