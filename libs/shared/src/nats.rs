use std::{future::Future, pin::Pin};

use async_nats::Client;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub host: String,
}

impl From<Configuration> for Pin<Box<dyn Future<Output = anyhow::Result<Client>> + Send>> {
    fn from(value: Configuration) -> Self {
        Box::pin(async move { Ok(async_nats::connect(value.host).await?) })
    }
}
