use async_nats::Client;
use serde::Deserialize;
use std::future::Future;

use crate::error::GenericError;

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

// todo: Prefer From since it automatically gives a free Into implementation
// Allows the configuration to directly create a nats connection
impl NatsConfiguration {
    pub async fn to_client(self) -> Result<Client, GenericError> {
        Ok(async_nats::connect(self.host).await?)
    }
}