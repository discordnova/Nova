use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use serde::de::DeserializeSeed;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace_span;
use twilight_model::gateway::event::{DispatchEvent, DispatchEventWithTypeDeserializer};

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct DispatchEventTagged(pub DispatchEvent);

impl Deref for DispatchEventTagged {
    type Target = DispatchEvent;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DispatchEventTagged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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
        Ok(Self(dispatch_event))
    }
}

impl Serialize for DispatchEventTagged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let _s = trace_span!("serializing DispatchEventTagged");
        let kind = self.0.kind().name().unwrap();
        DispatchEventTaggedSerialized {
            data: serde_json::to_value(&self.0).unwrap(),
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
#[cfg(test)]
mod tests {
    use serde_json::json;
    use twilight_model::gateway::event::DispatchEvent;

    use super::DispatchEventTagged;

    #[test]
    fn serialize_event_tagged() {
        let dispatch_event = DispatchEvent::GiftCodeUpdate;

        let value = serde_json::to_value(&dispatch_event);
        assert!(value.is_ok());
        let value = value.unwrap();

        let kind = value.get("t").and_then(serde_json::Value::as_str);
        assert_eq!(kind, Some("GIFT_CODE_UPDATE"));
    }

    #[test]
    fn deserialize_event_tagged() {
        let json = json!({
            "t": "GIFT_CODE_UPDATE",
            "d": {}
        });

        let dispatch_event = serde_json::from_value::<DispatchEventTagged>(json);
        assert!(dispatch_event.is_ok());

        let dispatch_event_tagged = dispatch_event.unwrap();

        assert_eq!(
            DispatchEventTagged(DispatchEvent::GiftCodeUpdate),
            dispatch_event_tagged
        );
    }
}
