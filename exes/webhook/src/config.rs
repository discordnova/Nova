use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ServerSettings {
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Discord {
    pub public_key: String,
    pub client_id: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub server: ServerSettings,
    pub discord: Discord,
}
