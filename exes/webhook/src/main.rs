mod config;
mod handler;
use std::{future::Future, pin::Pin};

use crate::{
    config::WebhookConfig,
    handler::{handler::WebhookService, make_service::MakeSvc},
};
use hyper::Server;
use leash::{ignite, AnyhowResultFuture, Component};
use shared::{config::Settings, log::info, nats_crate::Client};

#[derive(Clone, Copy)]
struct WebhookServer {}

impl Component for WebhookServer {
    type Config = WebhookConfig;
    const SERVICE_NAME: &'static str = "webhook";

    fn start(&self, settings: Settings<Self::Config>) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            info!("Starting server on {}", settings.server.listening_adress);

            let bind = settings.server.listening_adress;
            let nats =
                Into::<Pin<Box<dyn Future<Output = anyhow::Result<Client>>>>>::into(settings.nats)
                    .await?;

            let make_service = MakeSvc::new(WebhookService {
                config: settings.config,
                nats: nats.clone(),
            });

            let server = Server::bind(&bind).serve(make_service);

            server.await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}

ignite!(WebhookServer);
