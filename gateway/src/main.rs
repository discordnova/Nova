mod client;
mod config;
mod cluster_manager;

use common::config::Settings;

#[tokio::main]
async fn main() {    
    let settings: Settings<config::Config> = Settings::new("gateway").unwrap();
    let manager = cluster_manager::ClusterManager::new(settings.config);
    manager.start().await;
}
