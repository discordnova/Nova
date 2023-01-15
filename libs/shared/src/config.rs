use anyhow::Result;
use config::{Config, Environment, File};
use serde::{de::DeserializeOwned, Deserialize};
use std::{env, ops::Deref};
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings<T: Clone + DeserializeOwned> {
    #[serde(skip_deserializing)]
    pub config: T,
    pub nats: crate::nats::Configuration,
    pub redis: crate::redis::Configuration,
    pub opentelemetry: Option<crate::opentelemetry::Configuration>,
}

impl<T: Clone + DeserializeOwned + Default> Settings<T> {
    /// # Errors
    /// Fails it the config could not be deserialized to `Self::T`
    pub fn new(service_name: &str) -> Result<Self> {
        let mut builder = Config::builder();

        builder = builder.add_source(File::with_name("config/default"));
        let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
        info!("Configuration Environment: {}", mode);

        builder = builder.add_source(File::with_name(&format!("config/{mode}")).required(false));
        builder = builder.add_source(File::with_name("config/local").required(false));

        let env = Environment::with_prefix("NOVA").separator("__");
        // we can configure each component using environment variables
        builder = builder.add_source(env);

        let config = builder.build()?;
        let mut settings: Self = config.clone().try_deserialize()?;

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
