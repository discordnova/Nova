use std::net::ToSocketAddrs;
mod handle;
mod config;

use hyper::Server;
use log::info;
use handle::MakeSvc;
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
    let nats = Box::new(nats::connect("localhost").unwrap());
    let server = Server::bind(&addr).serve(MakeSvc {
        settings: settings.config.clone(),
        nats
    });

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
