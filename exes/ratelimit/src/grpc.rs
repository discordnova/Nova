
use std::pin::Pin;

use futures_util::Stream;
use proto::nova::ratelimit::ratelimiter::{ratelimiter_server::Ratelimiter, BucketSubmitTicketResponse, BucketSubmitTicketRequest};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
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

#[tonic::async_trait]
impl Ratelimiter for RLServer {

    type SubmitTicketStream =
        Pin<Box<dyn Stream<Item = Result<BucketSubmitTicketResponse, Status>> + Send>>;

    async fn submit_ticket(
        &self,
        req: Request<Streaming<BucketSubmitTicketRequest>>,
    ) -> Result<Response<Self::SubmitTicketStream>, Status> {
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
                        let a = imrl.ticket(path).await.unwrap();
                        receiver = Some(a);
                        

                        tx.send(Ok(BucketSubmitTicketResponse {
                            accepted: 1
                        })).await.unwrap();

                    },
                    proto::nova::ratelimit::ratelimiter::bucket_submit_ticket_request::Data::Headers(b) => {
                        if let Some(recv) = receiver {
                            let recv = recv.await.unwrap();
                            let rheaders = RatelimitHeaders::from_pairs(b.headers.iter().map(|f| (f.0.as_str(), f.1.as_bytes()))).unwrap();
                            
                            recv.headers(Some(rheaders)).unwrap();

                            break;
                        }
                    },
                }
            }
            println!("\tstream ended");
        });

        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::SubmitTicketStream
        ))
    }
}