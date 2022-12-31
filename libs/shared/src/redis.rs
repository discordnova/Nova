use redis::Client;
use serde::Deserialize;


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
