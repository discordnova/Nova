use std::fmt::Error;

use redis;

pub struct RedisClient {
    _client: redis::cluster::ClusterClient,
    pub conn: redis::cluster::ClusterConnection,
}

#[derive(Debug, Default)]
impl RedisClient {
    // Creates a RedisClient by providing him nodes
    fn new(nodes: Vec<&str>) -> Result<RedisClient, Error> {
        let client: redis::cluster::ClusterClient = redis::cluster::ClusterClient::open(nodes)
            .unwrap(
                "Unable to create a cluster client and open connections to {}",
                nodes,
            )?;
        let conn: redis::cluster::ClusterConnection = client
            .get_connection()
            .unwrap("Unable to get connections")?;

        Ok(RedisClient {
            _client: client,
            conn,
        })
    }

    /// Creates a RedisClient with redis://127.0.0.1/ as node
    fn default() -> Result<RedisClient, Error> {
        RedisClient::new(vec!["redis://127.0.0.1/"])
    }
}
