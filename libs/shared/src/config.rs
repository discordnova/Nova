use config::{Config, Environment, File};
use serde::{de::DeserializeOwned, Deserialize};
use std::{env, ops::Deref};
use tracing::info;

use crate::error::GenericError;
#[derive(Debug, Deserialize, Clone)]
pub struct Settings<T: Clone + DeserializeOwned + Default> {
    #[serde(skip_deserializing)]
    pub config: T,
    pub nats: crate::nats::NatsConfiguration,
    pub redis: crate::redis::RedisConfiguration,
}

impl<T: Clone + DeserializeOwned + Default> Settings<T> {
    pub fn new(service_name: &str) -> Result<Settings<T>, GenericError> {
        let mut builder = Config::builder();

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

        Ok(settings)
    }
}

impl<T: Clone + DeserializeOwned + Default> Deref for Settings<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
