use std::{net::ToSocketAddrs, sync::Arc};
mod config;
mod handler;
use crate::handler::make_service::MakeSvc;

use crate::config::Config;
use ed25519_dalek::PublicKey;
use hyper::Server;
use shared::config::Settings;
use shared::log::{error, info};

#[tokio::main]
async fn main() {
    let settings: Settings<Config> = Settings::new("webhook").unwrap();
    start(settings).await;
}

async fn start(settings: Settings<Config>) {
    let addr = format!(
        "{}:{}",
        settings.config.server.address, settings.config.server.port
    )
    .to_socket_addrs()
    .unwrap()
    .next()
    .unwrap();

    info!(
        "Starting server on {}:{}",
        settings.config.server.address, settings.config.server.port
    );

    let config = Arc::new(settings.config);
    let public_key =
        Arc::new(PublicKey::from_bytes(&hex::decode(&config.discord.public_key).unwrap()).unwrap());
    let server = Server::bind(&addr).serve(MakeSvc {
        settings: config,
        nats: Arc::new(settings.nats.to_client().await.unwrap()),
        public_key: public_key,
    });

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
