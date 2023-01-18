use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use async_nats::Client;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub host: String,
}

impl IntoFuture for Configuration {
    type Output = anyhow::Result<Client>;

    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { Ok(async_nats::connect(self.host).await?) })
    }
}
