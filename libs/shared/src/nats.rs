use std::{future::Future, pin::Pin};

use async_nats::Client;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfigurationClientCert {
    pub cert: String,
    pub key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfigurationTls {
    pub mtu: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfiguration {
    pub host: String,
}

impl From<NatsConfiguration> for Pin<Box<dyn Future<Output = anyhow::Result<Client>>>> {
    fn from(value: NatsConfiguration) -> Self {
        Box::pin(async move { Ok(async_nats::connect(value.host).await?) })
    }
}
