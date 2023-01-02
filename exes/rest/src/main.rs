use config::ReverseProxyConfig;

use handler::handle_request;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Server,
};
use hyper_tls::HttpsConnector;
use leash::{ignite, AnyhowResultFuture, Component};
use shared::config::Settings;
use std::convert::Infallible;

mod config;
mod handler;
mod ratelimit_client;

struct ReverseProxyServer {}
impl Component for ReverseProxyServer {
    type Config = ReverseProxyConfig;
    const SERVICE_NAME: &'static str = "rest";

    fn start(&self, settings: Settings<Self::Config>) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            // Client to the remote ratelimiters
            let ratelimiter = ratelimit_client::RemoteRatelimiter::new();
            let client = Client::builder().build(HttpsConnector::new());

            let service_fn = make_service_fn(move |_: &AddrStream| {
                let client = client.clone();
                let ratelimiter = ratelimiter.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                        let client = client.clone();
                        let ratelimiter = ratelimiter.clone();
                        async move {
                            handle_request(client, ratelimiter, "token".to_string(), request).await
                        }
                    }))
                }
            });

            let server = Server::bind(&settings.config.server.listening_adress).serve(service_fn);

            server.await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}

ignite!(ReverseProxyServer);
