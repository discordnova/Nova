use redis::{aio::MultiplexedConnection, Client};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfiguration {
    pub url: String,
}

impl From<RedisConfiguration>
    for Pin<Box<dyn Future<Output = anyhow::Result<MultiplexedConnection>> + Send>>
{
    fn from(value: RedisConfiguration) -> Self {
        Box::pin(async move {
            let con = Client::open(value.url)?;
            let (multiplex, ready) = con.create_multiplexed_tokio_connection().await?;

            tokio::spawn(ready);

            Ok(multiplex)
        })
    }
}
