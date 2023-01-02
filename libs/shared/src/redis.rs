use redis::{aio::MultiplexedConnection, Client};
use serde::Deserialize;
use std::{future::Future, pin::Pin};

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfiguration {
    pub url: String,
}

// Allows the configuration to directly create a nats connection
impl Into<Client> for RedisConfiguration {
    fn into(self) -> Client {
        redis::Client::open(self.url).unwrap()
    }
}

impl From<RedisConfiguration>
    for Pin<Box<dyn Future<Output = anyhow::Result<MultiplexedConnection>>>>
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
