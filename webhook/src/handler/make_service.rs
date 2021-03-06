use super::handler::HandlerService;
use crate::config::Config;
use hyper::service::Service;
use common::nats_crate::Connection;
use std::{
    future::{ready, Ready},
    sync::Arc,
    task::{Context, Poll},
};
use ed25519_dalek::PublicKey;

pub struct MakeSvc {
    pub settings: Arc<Config>,
    pub nats: Arc<Connection>,
    pub public_key: Arc<PublicKey>
}

impl<T> Service<T> for MakeSvc {
    type Response = HandlerService;
    type Error = std::io::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        ready(Ok(HandlerService {
            config: self.settings.clone(),
            nats: self.nats.clone(),
            public_key: self.public_key.clone()
        }))
    }
}
