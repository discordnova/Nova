use crate::{config::Config, ratelimit::Ratelimiter};
use futures_util::future::TryFutureExt;
use hyper::{
    client::HttpConnector, header::HeaderValue, http::uri::Parts, service::Service, Body, Client,
    HeaderMap, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use std::{future::Future, pin::Pin, sync::Arc, task::Poll};

#[derive(Clone)]
pub struct ServiceProxy {
    client: Client<HttpsConnector<HttpConnector>>,
    ratelimiter: Arc<Ratelimiter>,
    config: Arc<Config>,
}

impl Service<Request<Body>> for ServiceProxy {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.client.poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, mut req: Request<hyper::Body>) -> Self::Future {
        let host = "discord.com";
        let mut new_parts = Parts::default();

        let path = req.uri().path().to_string();

        new_parts.scheme = Some("https".parse().unwrap());
        new_parts.authority = Some(host.parse().unwrap());
        new_parts.path_and_query = Some(path.parse().unwrap());

        *req.uri_mut() = Uri::from_parts(new_parts).unwrap();

        let mut headers = HeaderMap::default();

        headers.insert("Host", HeaderValue::from_str("discord.com").unwrap());
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bot {}", self.config.discord.token)).unwrap(),
        );

        *req.headers_mut() = headers;
        let res = self.client
            .request(req)
            .map_ok(move |res| {
                if let Some(bucket) = res.headers().get("x-ratelimit-bucket") {
                    
                    println!("bucket ratelimit! {:?} : {:?}", path, bucket);
                }

                res
            });
        
        return Box::pin(res);
    }
}

impl ServiceProxy {
    pub fn new(config: Arc<Config>, ratelimiter: Arc<Ratelimiter>) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        ServiceProxy { client, config, ratelimiter }
    }
}
