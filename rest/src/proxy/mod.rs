use crate::{config::Config, ratelimit::Ratelimiter};
use common::{log::debug, prometheus::{Counter, HistogramVec, labels, opts, register_counter, register_histogram_vec}};
use hyper::{
    client::HttpConnector, header::HeaderValue, http::uri::Parts, service::Service, Body, Client,
    Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use tokio::sync::Mutex;
use std::{future::Future, pin::Pin, sync::Arc, task::Poll};

lazy_static::lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "nova_rest_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();

    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "nova_rest_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();

    static ref HTTP_COUNTER_STATUS: Counter = register_counter!(opts!(
        "nova_rest_http_requests_status",
        "Number of HTTP requests made by status",
        labels! {"" => ""}
    ))
    .unwrap();
}


#[derive(Clone)]
pub struct ServiceProxy {
    client: Client<HttpsConnector<HttpConnector>>,
    ratelimiter: Arc<Ratelimiter>,
    config: Arc<Config>,
    fail: Arc<Mutex<i32>>,
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
        HTTP_COUNTER.inc();

        let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();
        let host = "discord.com";
        let mut new_parts = Parts::default();

        let path = req.uri().path().to_string();

        new_parts.scheme = Some("https".parse().unwrap());
        new_parts.authority = Some(host.parse().unwrap());
        new_parts.path_and_query = Some(path.parse().unwrap());

        *req.uri_mut() = Uri::from_parts(new_parts).unwrap();

        let headers = req.headers_mut();
        headers.remove("user-agent");
        headers.insert("Host", HeaderValue::from_str("discord.com").unwrap());
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bot {}", self.config.discord.token)).unwrap(),
        );

        println!("{:?}", headers);

        let client = self.client.clone();
        let ratelimiter = self.ratelimiter.clone();
        let fail = self.fail.clone();

        return Box::pin(async move {
            let resp = match ratelimiter.before_request(&req).await {
                Ok(allowed) => match allowed {
                    crate::ratelimit::RatelimiterResponse::Ratelimited => {
                        debug!("ratelimited");
                        Ok(Response::builder().body("ratelimited".into()).unwrap())
                    }
                    _ => {
                        debug!("forwarding request");
                        *req.version_mut() = hyper::Version::HTTP_11;
                        match client.request(req).await {
                            Ok(mut response) => {
                                ratelimiter.after_request(&path, &response).await;
                                if response.status() != 200 {
                                    *fail.lock().await += 1
                                }
                                response.headers_mut().insert("x-fails", HeaderValue::from_str(&format!("{}", fail.lock().await)).unwrap());
                                response.headers_mut().remove("Connection");
                                response.headers_mut().remove("Keep-Alive");
                                response.headers_mut().remove("Proxy-Authenticate");
                                response.headers_mut().remove("Proxy-Authorization");
                                response.headers_mut().remove("Te");
                                response.headers_mut().remove("Trailers");
                                response.headers_mut().remove("Upgrade");
                                response.headers_mut().remove("Transfer-Encoding");
                                
                                Ok(response)
                            }
                            Err(e) => Err(e),
                        }
                    }
                },
                Err(e) => Ok(Response::builder()
                    .body(format!("server error: {}", e).into())
                    .unwrap()),
            };
            timer.observe_duration();
            resp
        });
    }
}

impl ServiceProxy {
    pub fn new(config: Arc<Config>, ratelimiter: Arc<Ratelimiter>) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let fail = Arc::new(Mutex::new(0));
        ServiceProxy {
            client,
            config,
            ratelimiter,
            fail
        }
    }
}
