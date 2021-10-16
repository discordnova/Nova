use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct Interaction {
    #[serde(rename = "type")]
    pub t: i16,
    pub data: Option<Value>,
}
