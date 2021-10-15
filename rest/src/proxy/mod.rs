use crate::{config::Config, ratelimit::Ratelimiter};
use common::log::debug;
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

impl ServiceProxy {
    async fn proxy_call() {}
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
        let client = self.client.clone();
        let ratelimiter = self.ratelimiter.clone();

        return Box::pin(async move {
            match ratelimiter.before_request(&req).await {
                Ok(allowed) => match allowed {
                    crate::ratelimit::RatelimiterResponse::Ratelimited => {
                        debug!("ratelimited");
                        Ok(Response::builder().body("ratelimited".into()).unwrap())
                    }
                    _ => {
                        debug!("forwarding request");
                        match client.request(req).await {
                            Ok(response) => {
                                ratelimiter.after_request(&path, &response).await;
                                Ok(response)
                            }
                            Err(e) => Err(e),
                        }
                    }
                },
                Err(e) => Ok(Response::builder()
                    .body(format!("server error: {}", e).into())
                    .unwrap()),
            }
        });
    }
}

impl ServiceProxy {
    pub fn new(config: Arc<Config>, ratelimiter: Arc<Ratelimiter>) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        ServiceProxy {
            client,
            config,
            ratelimiter,
        }
    }
}
