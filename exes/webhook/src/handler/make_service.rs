use hyper::service::Service;
use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

pub struct MakeSvc<T: Clone> {
    pub service: T,
}

impl<T, V: Clone> Service<T> for MakeSvc<V> {
    type Response = V;
    type Error = std::io::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        ready(Ok(self.service.clone()))
    }
}

impl<T: Clone> MakeSvc<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }
}
