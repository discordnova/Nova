use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, Default)]
pub struct CacheConfiguration {
    pub toggles: Vec<String>
}
