use crate::config::ReverseProxyConfig;

use self::remote_hashring::{HashRingWrapper, MetadataMap, VNode};
use opentelemetry::global;
use proto::nova::ratelimit::ratelimiter::bucket_submit_ticket_request::{Data, Headers};
use proto::nova::ratelimit::ratelimiter::BucketSubmitTicketRequest;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use std::time::{Duration, SystemTime};
use tokio::sync::oneshot::{self};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tracing::{debug, debug_span, instrument, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

mod remote_hashring;

#[derive(Clone, Debug)]
pub struct RemoteRatelimiter {
    remotes: Arc<RwLock<HashRingWrapper>>,
    stop: Arc<tokio::sync::broadcast::Sender<()>>,
    config: ReverseProxyConfig,
}

impl Drop for RemoteRatelimiter {
    fn drop(&mut self) {
        self.stop.clone().send(()).unwrap();
    }
}

type IssueTicket = Pin<
    Box<
        dyn Future<Output = anyhow::Result<oneshot::Sender<HashMap<String, String>>>>
            + Send
            + 'static,
    >,
>;

impl RemoteRatelimiter {
    async fn get_ratelimiters(&self) -> Result<(), anyhow::Error> {
        // get list of dns responses
        let responses = dns_lookup::lookup_host(&self.config.ratelimiter_address)
            .unwrap()
            .into_iter()
            .map(|f| f.to_string());

        let mut write = self.remotes.write().await;

        for ip in responses {
            let a = VNode::new(ip, self.config.ratelimiter_port).await?;
            write.add(a.clone());
        }

        Ok(())
    }

    #[must_use]
    pub fn new(config: ReverseProxyConfig) -> Self {
        let (rx, mut tx) = broadcast::channel(1);
        let obj = Self {
            remotes: Arc::new(RwLock::new(HashRingWrapper::default())),
            stop: Arc::new(rx),
            config,
        };

        let obj_clone = obj.clone();
        // Task to update the ratelimiters in the background
        tokio::spawn(async move {
            loop {
                let sleep = tokio::time::sleep(Duration::from_secs(10));
                tokio::pin!(sleep);

                debug!("refreshing");
                obj_clone.get_ratelimiters().await.unwrap();
                tokio::select! {
                    () = &mut sleep => {
                        debug!("timer elapsed");
                    },
                    _ = tx.recv() => {}
                }
            }
        });

        obj
    }

    #[instrument(name = "ticket task")]
    pub fn ticket(&self, path: String) -> IssueTicket {
        let remotes = self.remotes.clone();
        let (tx, rx) = oneshot::channel::<HashMap<String, String>>();
        Box::pin(
            async move {
                // Get node managing this path
                let mut node = (*remotes.read().await.get(&path).unwrap()).clone();

                // Buffers for the gRPC streaming channel.
                let (send, remote) = mpsc::channel(5);
                let (do_request, wait) = oneshot::channel();
                // Tonic requires a stream to be used; Since we use a mpsc channel, we can create a stream from it
                let stream = ReceiverStream::new(remote);

                let mut request = Request::new(stream);

                let span = debug_span!("remote request");
                let context = span.context();
                global::get_text_map_propagator(|propagator| {
                    propagator.inject_context(&context, &mut MetadataMap(request.metadata_mut()))
                });

                // Start the grpc streaming
                let ticket = node.submit_ticket(request).await?;

                // First, send the request
                send.send(BucketSubmitTicketRequest {
                    data: Some(Data::Path(path)),
                })
                .await?;

                // We continuously listen for events in the channel.
                let span = debug_span!("stream worker");
                tokio::spawn(
                    async move {
                        let span = debug_span!("waiting for ticket upstream");
                        let message = ticket
                            .into_inner()
                            .message()
                            .instrument(span)
                            .await
                            .unwrap()
                            .unwrap();

                        if message.accepted == 1 {
                            debug!("request ticket was accepted");
                            do_request.send(()).unwrap();
                            let span = debug_span!("waiting for response headers");
                            let headers = rx.instrument(span).await.unwrap();

                            send.send(BucketSubmitTicketRequest {
                                data: Some(Data::Headers(Headers {
                                    precise_time: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .expect("time went backwards")
                                        .as_millis()
                                        as u64,
                                    headers,
                                })),
                            })
                            .await
                            .unwrap();
                        }
                    }
                    .instrument(span),
                );

                // Wait for the message to be sent
                wait.await?;

                Ok(tx)
            }
            .instrument(Span::current()),
        )
    }
}
