use config::ReverseProxyConfig;

use handler::handle_request;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Server,
};
use leash::{AnyhowResultFuture, Component};
use opentelemetry::{global, trace::Tracer};
use opentelemetry_http::HeaderExtractor;
use shared::config::Settings;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::oneshot;

mod config;
mod handler;
mod ratelimit_client;

pub struct ReverseProxyServer {}
impl Component for ReverseProxyServer {
    type Config = ReverseProxyConfig;
    const SERVICE_NAME: &'static str = "rest";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            // Client to the remote ratelimiters
            let ratelimiter = ratelimit_client::RemoteRatelimiter::new(settings.config.clone());
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .build();

            let client: Client<_, hyper::Body> = Client::builder().build(https);
            let token = Arc::new(settings.discord.token.clone());
            let service_fn = make_service_fn(move |_: &AddrStream| {
                let client = client.clone();
                let ratelimiter = ratelimiter.clone();
                let token = token.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                        let parent_cx = global::get_text_map_propagator(|propagator| {
                            propagator.extract(&HeaderExtractor(request.headers()))
                        });
                        let _span =
                            global::tracer("").start_with_context("handle_request", &parent_cx);

                        let client = client.clone();
                        let ratelimiter = ratelimiter.clone();
                        let token = token.clone();
                        async move {
                            let token = token.as_str();
                            handle_request(client, ratelimiter, token, request).await
                        }
                    }))
                }
            });

            let server = Server::bind(&settings.config.server.listening_adress).serve(service_fn);

            server
                .with_graceful_shutdown(async {
                    stop.await.expect("should not fail");
                })
                .await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}
