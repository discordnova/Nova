use std::{net::ToSocketAddrs, sync::Arc};
mod config;
mod handler;
use crate::handler::make_service::MakeSvc;

use crate::config::Config;
use common::config::Settings;
use common::log::{error, info};
use hyper::Server;

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
    let server = Server::bind(&addr).serve(MakeSvc {
        settings: config,
        nats: Arc::new(settings.nats.into()),
    });

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
