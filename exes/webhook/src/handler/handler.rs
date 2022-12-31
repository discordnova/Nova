use super::error::WebhookError;
use super::signature::validate_signature;
use crate::config::Config;
use ed25519_dalek::PublicKey;
use hyper::{
    body::{to_bytes, Bytes},
    service::Service,
    Body, Method, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use shared::nats_crate::Client;
use shared::{
    log::{debug, error},
    payloads::{CachePayload, DispatchEventTagged, Tracing},
};
use std::{
    future::Future,
    pin::Pin,
    str::from_utf8,
    sync::Arc,
    task::{Context, Poll},
};
use twilight_model::gateway::event::{DispatchEvent};
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    gateway::payload::incoming::InteractionCreate,
};

/// Hyper service used to handle the discord webhooks
#[derive(Clone)]
pub struct HandlerService {
    pub config: Arc<Config>,
    pub nats: Arc<Client>,
    pub public_key: Arc<PublicKey>,
}

impl HandlerService {
    async fn check_request(&self, req: Request<Body>) -> Result<Bytes, WebhookError> {
        if req.method() == Method::POST {
            let signature = if let Some(sig) = req.headers().get("X-Signature-Ed25519") {
                sig.to_owned()
            } else {
                return Err(WebhookError::new(
                    StatusCode::BAD_REQUEST,
                    "missing signature header",
                ));
            };

            let timestamp = if let Some(timestamp) = req.headers().get("X-Signature-Timestamp") {
                timestamp.to_owned()
            } else {
                return Err(WebhookError::new(
                    StatusCode::BAD_REQUEST,
                    "missing timestamp header",
                ));
            };
            let data = to_bytes(req.into_body()).await?;

            if validate_signature(
                &self.public_key,
                &[timestamp.as_bytes().to_vec(), data.to_vec()].concat(),
                signature.to_str()?,
            ) {
                Ok(data)
            } else {
                Err(WebhookError::new(
                    StatusCode::UNAUTHORIZED,
                    "invalid signature",
                ))
            }
        } else {
            Err(WebhookError::new(StatusCode::NOT_FOUND, "not found"))
        }
    }

    async fn process_request(
        &mut self,
        req: Request<Body>,
    ) -> Result<Response<Body>, WebhookError> {
        match self.check_request(req).await {
            Ok(data) => {
                let utf8 = from_utf8(&data);
                match utf8 {
                    Ok(data) => match serde_json::from_str::<Interaction>(data) {
                        Ok(value) => {
                            match value.kind {
                                InteractionType::Ping => Ok(Response::builder()
                                    .header("Content-Type", "application/json")
                                    .body(serde_json::to_string(&Ping { t: 1 }).unwrap().into())
                                    .unwrap()),
                                _ => {
                                    debug!("calling nats");
                                    // this should hopefully not fail ?

                                    let data = CachePayload {
                                        tracing: Tracing {
                                            node_id: "".to_string(),
                                            span: None,
                                        },
                                        data: DispatchEventTagged {
                                            data: DispatchEvent::InteractionCreate(Box::new(
                                                InteractionCreate(value),
                                            )),
                                        },
                                    };

                                    let payload = serde_json::to_string(&data).unwrap();

                                    match self.nats.request(
                                        "nova.cache.dispatch.INTERACTION_CREATE".to_string(),
                                        Bytes::from(payload),
                                    ).await {
                                        Ok(response) => Ok(Response::builder()
                                            .header("Content-Type", "application/json")
                                            .body(Body::from(response.reply.unwrap()))
                                            .unwrap()),

                                        Err(error) => {
                                            error!("failed to request nats: {}", error);
                                            Err(WebhookError::new(
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                                "failed to request nats",
                                            ))
                                        }
                                    }
                                }
                            }
                        }

                        Err(error) => {
                            error!("invalid json body: {}", error);
                            Err(WebhookError::new(
                                StatusCode::BAD_REQUEST,
                                "invalid json body",
                            ))
                        }
                    },

                    Err(_) => Err(WebhookError::new(StatusCode::BAD_REQUEST, "not utf-8 body")),
                }
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
    #[serde(rename = "type")]
    t: i32,
}

/// Implementation of the service
impl Service<Request<Body>> for HandlerService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut clone = self.clone();
        Box::pin(async move {
            let response = clone.process_request(req).await;

            match response {
                Ok(r) => Ok(r),
                Err(e) => Ok(e.into()),
            }
        })
    }
}
