use super::{signature::validate_signature, types::Interaction};
use crate::config::Config;
use hyper::{Body, Method, Request, Response, StatusCode, body::{to_bytes, Bytes}, service::Service};
use log::{error, info, trace};
use nats::Connection;
use serde::{Deserialize, Serialize};
use std::{future::Future, io::{Error, ErrorKind}, pin::Pin, str::from_utf8, sync::Arc, task::{Context, Poll}};

/// Hyper service used to handle the discord webhooks
#[derive(Clone)]
pub struct HandlerService {
    pub config: Config,
    pub nats: Arc<Connection>,
}

impl HandlerService {
    async fn check_request(&self, req: Request<Body>) -> Result<Bytes, Error> {
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
                            Err(Error::new(
                                ErrorKind::InvalidData,
                                "invalid signature specified",
                            ))
                        }
                    } else {
                        Err(Error::new(
                            ErrorKind::BrokenPipe,
                            "failed to read signature",
                        ))
                    }
                } else {
                    Err(Error::new(ErrorKind::BrokenPipe, "unable to read body"))
                }
            } else {
                Err(Error::new(ErrorKind::InvalidData, "missing headers"))
            }
        } else {
            Err(Error::new(ErrorKind::InvalidData, "invalid method"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
    #[serde(rename = "type")]
    t: i32
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
        let self_clone = self.clone();

        Box::pin(async move {
            match self_clone.check_request(req).await {
                Ok(data) => {
                    let value: Interaction = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
                    trace!("received value: {:?}", value);

                    match value.t {
                        1 => {
                            info!("sending pong");
                            // a ping must be responded with another ping
                            return Ok(Response::builder().header("Content-Type", "application/json").body(serde_json::to_string(&Ping {
                                t: 1
                            }).unwrap().into()).unwrap());
                        },
                        _ => {
                            let payload = serde_json::to_string(&common::payloads::CachePayload {
                                tracing: common::payloads::Tracing {
                                    node_id: "".to_string(),
                                    span: None,
                                },
                                data: value,
                            }).unwrap();

                            match self_clone.nats.request("nova.cache.dispatch.interaction", payload) {
                                Ok(response) =>  {
                                    Ok(
                                        Response::builder()
                                            .header("Content-Type", "application/json")
                                            .body(from_utf8(&response.data).unwrap().to_string().into())
                                            .unwrap()
                                    )
                                },
                                Err(error) => {
                                    error!("failed to request nats: {}", error);
                                    Ok(
                                        Response::builder()
                                            .status(500)
                                            .body("an internal server error occured".to_string().into())
                                            .unwrap()
                                    )
                                }
                            }
                        },
                    }
                },
                Err(error) => {
                    Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body(error.to_string().into()).unwrap())
                }
            }
        })
    }
}
