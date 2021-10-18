use super::error::WebhookError;
use super::signature::validate_signature;
use crate::config::Config;
use common::discord_models::slash_commands::{Interaction, InteractionRequestType};
use common::log::{debug, error};
use common::nats_crate::Connection;
use common::payloads::CacheData;
use hyper::{
    body::{to_bytes, Bytes},
    service::Service,
    Body, Method, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    pin::Pin,
    str::from_utf8,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

/// Hyper service used to handle the discord webhooks
#[derive(Clone)]
pub struct HandlerService {
    pub config: Arc<Config>,
    pub nats: Arc<Connection>,
}

impl HandlerService {
    async fn check_request(&self, req: Request<Body>) -> Result<Bytes, WebhookError> {
        if req.method() == Method::POST {
            let headers = req.headers().clone();
            let signature = headers.get("X-Signature-Ed25519");
            let timestamp = headers.get("X-Signature-Timestamp");
            if let (Some(timestamp), Some(signature)) = (timestamp, signature) {
                if let Ok(data) = to_bytes(req.into_body()).await {
                    let contatenated_data = [timestamp.as_bytes().to_vec(), data.to_vec()].concat();
                    if let Ok(signature_str) = &signature.to_str() {
                        if validate_signature(
                            &self.config.discord.public_key,
                            &contatenated_data,
                            signature_str,
                        ) {
                            Ok(data)
                        } else {
                            Err(WebhookError::new(
                                StatusCode::UNAUTHORIZED,
                                "invalid signature",
                            ))
                        }
                    } else {
                        Err(WebhookError::new(
                            StatusCode::BAD_REQUEST,
                            "failed to read signature",
                        ))
                    }
                } else {
                    Err(WebhookError::new(
                        StatusCode::BAD_REQUEST,
                        "unable to read body",
                    ))
                }
            } else {
                Err(WebhookError::new(
                    StatusCode::UNAUTHORIZED,
                    "missing signature headers",
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
                        Ok(value) => match value.type_ {
                            InteractionRequestType::Ping => Ok(Response::builder()
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(&Ping { t: 1 }).unwrap().into())
                                .unwrap()),
                            _ => {
                                debug!("calling nats");
                                // this should hopefully not fail ?
                                let payload =
                                    serde_json::to_string(&common::payloads::CachePayload {
                                        tracing: common::payloads::Tracing {
                                            node_id: "".to_string(),
                                            span: None,
                                        },
                                        data: CacheData::InteractionCreate {
                                            interaction: Box::new(value),
                                        },
                                    })
                                    .unwrap();

                                match self.nats.request_timeout(
                                    "nova.cache.dispatch.interaction",
                                    payload,
                                    Duration::from_secs(2),
                                ) {
                                    Ok(response) => Ok(Response::builder()
                                        .header("Content-Type", "application/json")
                                        .body(Body::from(response.data))
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
                        },

                        Err(_) => Err(WebhookError::new(
                            StatusCode::BAD_REQUEST,
                            "invalid json body",
                        )),
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
