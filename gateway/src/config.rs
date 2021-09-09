use enumflags2::BitFlags;
use crate::client::structs::Intents;
use serde::Deserialize;

/// Config for the client connection.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ClusterClientConfig {
    pub token: String,
    pub large_threshold: Option<u64>,
    pub intents: BitFlags<Intents>
}

/// Configuration for the cluster manager
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ClusterClientSharding {
    pub cluster_size: i64,
    pub cluster_id: i64,
    pub shard_count: i64
}

/// Configuration for the output of the cluster
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ClusterRelay {
    pub relay_instances: i64
}

/// Configuration for the gateway component
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub discord: ClusterClientConfig,
    pub clustering: ClusterClientSharding,
    pub relaying: ClusterRelay,
}
