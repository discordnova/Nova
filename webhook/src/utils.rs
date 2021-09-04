use std::env;

use config::{Config, ConfigError, Environment, File};
use log::info;
use serde::Deserialize;

/// Executes the required configuration steps for the program,
/// uncluding build information, Sentry and logging.
pub fn setup_program (_name: &str) {
    pretty_env_logger::init();
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Discord {
    pub public_key: String,
    pub client_id: u32,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub discord: Discord,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut default = Config::default();
        default.merge(File::with_name("config/default"))?;
        let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
        info!("Configuration Environment: {}", mode);
        
        default.merge(File::with_name(&format!("config/{}", mode)).required(false))?;
        default.merge(File::with_name("config/local").required(false))?;
        default.merge(Environment::with_prefix("NOVA_GATEWAY"))?;
        
        println!("Debug mode: {:?}", default.get_bool("debug"));

        let config: Self = default.try_into().unwrap();

        Ok(config)
    }   
}
