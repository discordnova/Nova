use anyhow::Result;
use config::{Config, Environment, File};
use leash::Component;
use serde::de::DeserializeOwned;
use serde_json::Value;
use shared::config::Settings;
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot::Sender},
    task::JoinHandle,
};
use tracing::{
    debug,
    error, info,
};

/// Represents a all in one instance
pub struct AllInOneInstance {
    pub runtime: Runtime,
    pub(crate) handles: Vec<(&'static str, Sender<()>, JoinHandle<()>)>,
}

/// Loads the settings from a component using a string
fn load_settings_for<T: Default + DeserializeOwned + Clone>(
    settings: &str,
    name: &str,
) -> Result<Settings<T>> {
    let value: Value = serde_json::from_str(settings)?;
    let section: T = serde_json::from_value(value.get(name).unwrap().clone())?;
    let mut settings: Settings<T> = serde_json::from_value(value)?;
    settings.config = section;

    Ok(settings)
}

pub(crate) fn start_component<T: Component>(
    json: &str,
    error_sender: mpsc::Sender<anyhow::Error>,
    runtime: &Runtime,
) -> Result<(&'static str, Sender<()>, JoinHandle<()>)> {
    let name = T::SERVICE_NAME;
    let instance = T::new();

    // We setup stop signals
    let (stop, signal) = tokio::sync::oneshot::channel();
    let settings = load_settings_for(json, name)?;

    let handle = runtime.spawn(async move {
        debug!("starting component {}", name);
        match instance.start(settings, signal).await {
            Ok(_) => info!("Component {} gracefully exited", name),
            Err(error) => {
                error!("Component {} exited with error {}", name, error);
                error_sender
                    .send(error)
                    .await
                    .expect("Couldn't send the error notification to the error mpsc");
            }
        }
    });

    Ok((name, stop, handle))
}

pub(crate) fn load_config_file() -> Result<Value> {
    let mut builder = Config::builder();

    builder = builder.add_source(File::with_name("config/default"));
    let mode = std::env::var("ENV").unwrap_or_else(|_| "development".into());
    info!("Configuration Environment: {}", mode);

    builder = builder.add_source(File::with_name(&format!("config/{mode}")).required(false));
    builder = builder.add_source(File::with_name("config/local").required(false));

    let env = Environment::with_prefix("NOVA").separator("__");
    // we can configure each component using environment variables
    builder = builder.add_source(env);

    let config: Value = builder.build()?.try_deserialize()?;

    Ok(config)
}
