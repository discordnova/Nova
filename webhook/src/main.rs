use std::net::ToSocketAddrs;
use hyper::Server;
use log::info;

use crate::handle::MakeSvc;
extern crate log;

mod handle;
mod utils;


#[tokio::main]
async fn main() {
    utils::setup_program("webhook");
    let config = utils::Settings::new().unwrap();
    
    let addr = format!("{}:{}", config.server.address, config.server.port)
        .to_socket_addrs()
        .unwrap().next().unwrap();

    info!("Starting server on {}:{}", config.server.address, config.server.port);
    let server = Server::bind(&addr).serve(MakeSvc { settings: config.clone() });

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
