use std::{future::{Ready, ready}, sync::Arc, task::{Context, Poll}};
use hyper::service::Service;
use nats::Connection;
use crate::config::Config;
use super::handler::HandlerService;


pub struct MakeSvc {
    pub settings: Config,
    pub nats: Arc<Connection>,
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
        }))
    }
}
