use std::env;
use config::{Config, ConfigError, Environment, File, Source};
use log::info;
use serde::Deserialize;

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
}

/// 
impl<T> Settings<T>
where
    T: Deserialize<'static> + std::default::Default + Clone,
{
    /// Initializes a new configuration like the other components of nova
    /// And starts the prometheus metrics server if needed.
    pub fn new(service_name: &str) -> Result<Settings<T>, ConfigError> {
        let mut default = Config::default();
        // this file my be shared with all the components
        default.merge(File::with_name("config/default"))?;
        let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
        info!("Configuration Environment: {}", mode);

        default.merge(File::with_name(&format!("config/{}", mode)).required(false))?;
        default.merge(File::with_name("config/local").required(false))?;

        let env = Environment::with_prefix("NOVA").separator("__");
        println!("{:?}", env.collect());
        // we can configure each component using environment variables
        default.merge(env)?;
        let mut config: Settings<T> = default.clone().try_into().unwrap();

        //  try to load the config
        config.config = default.get::<T>(&service_name).unwrap();
        pretty_env_logger::init();

        // start the monitoring system if needed
        crate::monitoring::start_monitoring(&config.monitoring);
        Ok(config)
    }
}
