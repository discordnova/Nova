use std::{convert::Infallible, sync::Arc};

use crate::{config::Config, ratelimit::Ratelimiter};
use common::{
    config::Settings,
    log::{error, info},
    redis_crate::Client,
};
use hyper::{server::conn::AddrStream, service::make_service_fn, Server};
use std::net::ToSocketAddrs;
use tokio::sync::Mutex;

use crate::proxy::ServiceProxy;

mod config;
mod proxy;
mod ratelimit;

#[tokio::main]
async fn main() {
    let settings: Settings<Config> = Settings::new("rest").unwrap();
    let _guard = settings.sentry();
    let config = Arc::new(settings.config);
    let redis_client: Client = settings.redis.into();
    let redis = Arc::new(Mutex::new(
        redis_client.get_async_connection().await.unwrap(),
    ));
    let ratelimiter = Arc::new(Ratelimiter::new(redis));

    let addr = format!("{}:{}", config.server.address, config.server.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let service_fn = make_service_fn(move |_: &AddrStream| {
        let service_proxy = ServiceProxy::new(config.clone(), ratelimiter.clone());
        async move { Ok::<_, Infallible>(service_proxy) }
    });

    let server = Server::bind(&addr).serve(service_fn);

    info!("starting ratelimit server");
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
