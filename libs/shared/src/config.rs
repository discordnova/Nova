use std::env;
use config::{Config, Environment, File};
use log::info;
use serde::Deserialize;

use crate::error::GenericError;

/// Settings<T> is the base structure for all the nova's component config
/// you can specify a type T and the name of the component. the "config"
/// field will be equals to the key named after the given component name
/// and will be of type T
#[derive(Debug, Deserialize, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct Settings<T> {
    #[serde(skip_deserializing)]
    pub config: T,
    pub monitoring: crate::monitoring::MonitoringConfiguration,
    pub nats: crate::nats::NatsConfiguration,
    pub redis: crate::redis::RedisConfiguration,
}

/// 
impl<T> Settings<T>
where
    T: Deserialize<'static> + std::default::Default + Clone,
{

    /// Initializes a new configuration like the other components of nova
    /// And starts the prometheus metrics server if needed.
    pub fn new(service_name: &str) -> Result<Settings<T>, GenericError> {
        pretty_env_logger::init();

        let mut builder = Config::builder();
        
        // this file my be shared with all the components
        builder = builder.add_source(File::with_name("config/default"));
        let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
        info!("Configuration Environment: {}", mode);

        builder = builder.add_source(File::with_name(&format!("config/{}", mode)).required(false));
        builder = builder.add_source(File::with_name("config/local").required(false));

        let env = Environment::with_prefix("NOVA").separator("__");
        // we can configure each component using environment variables
        builder = builder.add_source(env);
        
        let config = builder.build()?;
        let mut settings: Settings<T> = config.clone().try_deserialize()?;

        //  try to load the config
        settings.config = config.get::<T>(service_name)?;
        pretty_env_logger::init();

        // start the monitoring system if needed
        crate::monitoring::start_monitoring(&settings.monitoring);
        Ok(settings)
    }
}

pub fn test_init() {
    pretty_env_logger::init();
}
