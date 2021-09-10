use crate::client::connexion::Connexion;
use crate::client::structs::*;
use crate::config::Config;
use log::info;
use futures::future::select_all;

pub struct ClusterManager {
    gateway_connexions: Vec<Connexion>,
    config: Config,
}

impl ClusterManager {

    pub fn new(config: crate::config::Config) -> ClusterManager {
        // create the memory for containing all the gateways
        let gateways: Vec<Connexion> = Vec::with_capacity(config.clustering.shard_count as usize);

        ClusterManager {
            gateway_connexions: gateways,
            config,
        }
    }

    pub async fn start(mut self) {
        info!("Starting cluster manager...");
        info!("I am cluster {}/{} managing {} discord shards", self.config.clustering.cluster_id+1, self.config.clustering.cluster_size, self.config.clustering.shard_count);

        for i in 0..self.config.clustering.shard_count {
            let shard_id = self.config.clustering.cluster_id * self.config.clustering.cluster_size + i;
            info!("Starting shard {} for cluster {} for discord shard {}", i, self.config.clustering.cluster_id, shard_id);
            self.gateway_connexions.push(Connexion::new(ClientConfig{
                token: self.config.discord.token.clone(),
                intents: self.config.discord.intents,
                large_threshold: self.config.discord.large_threshold,
                shard: Some(Sharding {
                    total_shards: self.config.clustering.cluster_size * self.config.clustering.shard_count,
                    current_shard: shard_id,
                }),
            }));
        }
        let tasks = self.gateway_connexions.into_iter().map(|item| {
            Box::pin(item.start())
        });
        let task = select_all(tasks).await;
        info!("one shard crashed, we need a restart {:?}", task.0);
    }
}