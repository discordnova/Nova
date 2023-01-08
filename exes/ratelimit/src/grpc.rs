use opentelemetry::{global, propagation::Extractor};
use proto::nova::ratelimit::ratelimiter::{
    ratelimiter_server::Ratelimiter, BucketSubmitTicketRequest, BucketSubmitTicketResponse,
};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tracing::{debug, debug_span, info, Instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use twilight_http_ratelimiting::{ticket::TicketReceiver, RatelimitHeaders};

use crate::redis_global_local_bucket_ratelimiter::RedisGlobalLocalBucketRatelimiter;

pub struct RLServer {
    ratelimiter: RedisGlobalLocalBucketRatelimiter,
}

impl RLServer {
    pub fn new(ratelimiter: RedisGlobalLocalBucketRatelimiter) -> Self {
        Self { ratelimiter }
    }
}

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

#[tonic::async_trait]
impl Ratelimiter for RLServer {
    type SubmitTicketStream =
        Pin<Box<dyn Stream<Item = Result<BucketSubmitTicketResponse, Status>> + Send>>;

    async fn submit_ticket(
        &self,
        req: Request<Streaming<BucketSubmitTicketRequest>>,
    ) -> Result<Response<Self::SubmitTicketStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata())));
        // Generate a tracing span as usual
        let span = tracing::span!(tracing::Level::INFO, "request process");

        // Assign parent trace from external context
        span.set_parent(parent_cx);

        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);
        let imrl = self.ratelimiter.clone();

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be drooped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            let mut receiver: Option<TicketReceiver> = None;
            while let Some(result) = in_stream.next().await {
                let result = result.unwrap();

                match result.data.unwrap() {
                    proto::nova::ratelimit::ratelimiter::bucket_submit_ticket_request::Data::Path(path) => {
                        let span = debug_span!("requesting ticket");
                        let a = imrl.ticket(path).instrument(span).await.unwrap();
                        receiver = Some(a);

                        tx.send(Ok(BucketSubmitTicketResponse {
                            accepted: 1
                        })).await.unwrap();
                    },
                    proto::nova::ratelimit::ratelimiter::bucket_submit_ticket_request::Data::Headers(b) => {
                        if let Some(recv) = receiver {
                            let span = debug_span!("waiting for headers data");
                            let recv = recv.instrument(span).await.unwrap();
                            let rheaders = RatelimitHeaders::from_pairs(b.headers.iter().map(|f| (f.0.as_str(), f.1.as_bytes()))).unwrap();

                            recv.headers(Some(rheaders)).unwrap();
                            break;
                        }
                    },
                }
            }

            debug!("\tstream ended");
            info!("request terminated");
        }.instrument(span));

        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::SubmitTicketStream
        ))
    }
}
