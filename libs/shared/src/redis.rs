use redis::{aio::MultiplexedConnection, Client};
use serde::Deserialize;
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub url: String,
}

impl IntoFuture for Configuration {
    type Output = anyhow::Result<MultiplexedConnection>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let con = Client::open(self.url)?;
            let (multiplex, ready) = con.create_multiplexed_tokio_connection().await?;

            tokio::spawn(ready);

            Ok(multiplex)
        })
    }
}
