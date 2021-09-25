use std::{net::ToSocketAddrs, sync::Arc};
mod handler;
mod config;
use crate::handler::make_service::MakeSvc;

use hyper::Server;
use log::{info, error};
use common::config::Settings;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let settings: Settings<Config> = Settings::new("webhook").unwrap();

    let addr = format!("{}:{}", settings.config.server.address, settings.config.server.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    info!(
        "Starting server on {}:{}",
        settings.config.server.address, settings.config.server.port
    );

    let server = Server::bind(&addr).serve(MakeSvc {
        settings: settings.config.clone(),
        nats: Arc::new(settings.nats.into()),
    });

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
