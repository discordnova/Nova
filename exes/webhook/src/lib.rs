#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    unsafe_code
)]

mod config;
mod handler;
use std::future::IntoFuture;

use crate::{
    config::Webhook,
    handler::{make_service::MakeSvc, WebhookService},
};
use hyper::Server;
use leash::{AnyhowResultFuture, Component};
use shared::config::Settings;
use tokio::sync::oneshot;
use tracing::info;
#[derive(Clone, Copy)]
pub struct WebhookServer {}

impl Component for WebhookServer {
    type Config = Webhook;
    const SERVICE_NAME: &'static str = "webhook";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            info!("Starting server on {}", settings.server.listening_adress);

            let bind = settings.server.listening_adress;
            info!("Nats connected!");
            let nats = settings.nats.into_future().await?;

            let make_service = MakeSvc::new(WebhookService {
                config: settings.config,
                nats: nats.clone(),
            });

            let server = Server::bind(&bind).serve(make_service);

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
