use std::env;

use config::{Config, ConfigError, Environment, File};
use log::info;
use serde::{Deserialize};


#[derive(Debug, Deserialize, Clone)]
#[serde(bound(deserialize = "T: Deserialize<'de> + std::default::Default + Clone"))]
pub struct Settings<T> {
    #[serde(skip_deserializing)]
    pub config: T,
    pub monitoring: crate::monitoring::MonitoringConfiguration,
}

impl<T> Settings<T> where T: Deserialize<'static> + std::default::Default + Clone {
    pub fn new(service_name: &str) -> Result<Settings<T>, ConfigError> {
        let mut default = Config::default();
        // this file my be shared with all the components
        default.merge(File::with_name("config/default"))?;
        let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
        info!("Configuration Environment: {}", mode);

        default.merge(File::with_name(&format!("config/{}", mode)).required(false))?;
        default.merge(File::with_name("config/local").required(false))?;

        // we can configure each component using environment variables
        default.merge(Environment::with_prefix(&format!("NOVA_{}", service_name)))?;
        let mut config: Settings<T> = default.clone().try_into().unwrap();

        //  try to load the config
        config.config = default.get::<T>(&service_name).unwrap();

        // setup the logger
        pretty_env_logger::init_custom_env(&format!("NOVA_{}_LOG", service_name));

        // start the monitoring system if needed
        crate::monitoring::start_monitoring(&config.monitoring);
        Ok(config)
    }
}
