use crate::config::Webhook;
use anyhow::bail;
use async_nats::Client;
use ed25519_dalek::PublicKey;
use hyper::{
    body::{to_bytes, Bytes},
    service::Service,
    Body, Method, Request, Response,
};
use shared::payloads::{CachePayload, DispatchEventTagged};
use signature::validate;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::{debug, error};
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    gateway::payload::incoming::InteractionCreate,
};

pub mod make_service;
mod signature;

#[cfg(test)]
pub mod tests;

/// Hyper service used to handle the discord webhooks
#[derive(Clone)]
pub struct WebhookService {
    pub config: Webhook,
    pub nats: Client,
}

impl WebhookService {
    async fn check_request(req: Request<Body>, pk: PublicKey) -> Result<Bytes, anyhow::Error> {
        if req.method() == Method::POST {
            let signature = if let Some(sig) = req.headers().get("X-Signature-Ed25519") {
                sig.clone()
            } else {
                bail!("Missing signature header");
            };

            let timestamp = if let Some(timestamp) = req.headers().get("X-Signature-Timestamp") {
                timestamp.clone()
            } else {
                bail!("Missing timestamp header");
            };
            let data = to_bytes(req.into_body()).await?;

            if validate(
                &pk,
                &[timestamp.as_bytes().to_vec(), data.to_vec()].concat(),
                signature.to_str()?,
            ) {
                Ok(data)
            } else {
                bail!("invalid signature");
            }
        } else {
            bail!("not found");
        }
    }

    async fn process_request(
        req: Request<Body>,
        nats: Client,
        pk: PublicKey,
    ) -> Result<Response<Body>, anyhow::Error> {
        let data = Self::check_request(req, pk).await?;
        let interaction: Interaction = serde_json::from_slice(&data)?;

        if interaction.kind == InteractionType::Ping {
            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .body(r#"{"type":1}"#.into())
                .unwrap())
        } else {
            debug!("calling nats");
            // this should hopefully not fail ?

            let data = CachePayload {
                data: DispatchEventTagged {
                    data: DispatchEvent::InteractionCreate(Box::new(InteractionCreate(
                        interaction,
                    ))),
                },
            };

            let payload = serde_json::to_string(&data).unwrap();

            match nats
                .request(
                    "nova.cache.dispatch.INTERACTION_CREATE".to_string(),
                    Bytes::from(payload),
                )
                .await
            {
                Ok(response) => Ok(Response::builder()
                    .header("Content-Type", "application/json")
                    .body(Body::from(response.payload))
                    .unwrap()),

                Err(error) => {
                    error!("failed to request nats: {}", error);
                    Err(anyhow::anyhow!("internal error"))
                }
            }
        }
    }
}

/// Implementation of the service
impl Service<hyper::Request<Body>> for WebhookService {
    type Response = hyper::Response<Body>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let future = Self::process_request(req, self.nats.clone(), self.config.discord.public_key);
        Box::pin(async move {
            let response = future.await;

            match response {
                Ok(r) => Ok(r),
                Err(e) => Err(e),
            }
        })
    }
}
