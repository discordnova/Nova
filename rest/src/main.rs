use std::{convert::Infallible, sync::Arc};

use crate::config::Config;
use common::{config::Settings, log::{error, info}};
use hyper::{server::conn::AddrStream, service::make_service_fn, Server};
use std::net::ToSocketAddrs;

use crate::proxy::ServiceProxy;

mod config;
mod proxy;

#[tokio::main]
async fn main() {
    let settings: Settings<Config> = Settings::new("rest").unwrap();
    let config = Arc::new(settings.config);

    let addr = format!("{}:{}", config.server.address, config.server.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let service_fn = make_service_fn(move |_: &AddrStream| {
        let service_proxy = ServiceProxy::new(config.clone());
        async move { Ok::<_, Infallible>(service_proxy) }
    });

    let server = Server::bind(&addr).serve(service_fn);

    info!("starting ratelimit server");
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
