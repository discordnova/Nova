use std::fmt::Debug;

use serde::de::DeserializeSeed;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace_span;
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

// todo(MatthieuCoder): Remove the use of the Value
impl<'de> Deserialize<'de> for DispatchEventTagged {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _s = trace_span!("deserializing DispatchEventTagged");
        let tagged = DispatchEventTaggedSerialized::deserialize(deserializer)?;
        let deserializer_seed = DispatchEventWithTypeDeserializer::new(&tagged.kind);
        let dispatch_event = deserializer_seed.deserialize(tagged.data).unwrap();
        Ok(Self {
            data: dispatch_event,
        })
    }
}

impl Serialize for DispatchEventTagged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let _s = trace_span!("serializing DispatchEventTagged");
        let kind = self.data.kind().name().unwrap();
        DispatchEventTaggedSerialized {
            data: serde_json::to_value(&self.data).unwrap(),
            kind: kind.to_string(),
        }
        .serialize(serializer)
    }
}

/// Payload send to the nova cache queues
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachePayload {
    #[serde(flatten)]
    pub data: DispatchEventTagged,
}
