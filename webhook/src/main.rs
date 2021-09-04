use std::net::ToSocketAddrs;

use hyper::Server;
use log::info;

extern crate log;
pub mod handle;
pub mod utils;

use utils::{setup_program, Settings};
use handle::MakeSvc;

#[tokio::main]
async fn main() {
    setup_program("webhook");
    let config = Settings::new().unwrap();
    
    let addr = format!("{}:{}", config.server.address, config.server.port)
        .to_socket_addrs()
        .unwrap().next().unwrap();

    info!("Starting server on {}:{}", config.server.address, config.server.port);
    let server = Server::bind(&addr).serve(MakeSvc { settings: config.clone() });

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
