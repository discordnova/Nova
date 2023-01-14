use crate::config::ReverseProxy;

use self::remote_hashring::{HashRingWrapper, MetadataMap, VNode};
use anyhow::anyhow;
use opentelemetry::global;
use proto::nova::ratelimit::ratelimiter::{BucketSubmitTicketRequest, HeadersSubmitRequest};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{broadcast, RwLock};
use tonic::Request;
use tracing::{debug, error, info_span, instrument, trace_span, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

mod remote_hashring;

#[derive(Clone, Debug)]
pub struct RemoteRatelimiter {
    remotes: Arc<RwLock<HashRingWrapper>>,
    current_remotes: Vec<String>,

    stop: Arc<tokio::sync::broadcast::Sender<()>>,
    config: ReverseProxy,
}

impl Drop for RemoteRatelimiter {
    fn drop(&mut self) {
        let _ = self
            .stop
            .clone()
            .send(())
            .map_err(|_| error!("ratelimiter was already stopped"));
    }
}

impl RemoteRatelimiter {
    async fn get_ratelimiters(&self) -> Result<(), anyhow::Error> {
        // get list of dns responses
        let responses: Vec<String> = dns_lookup::lookup_host(&self.config.ratelimiter_address)?
            .into_iter()
            .filter(std::net::IpAddr::is_ipv4)
            .map(|address| address.to_string())
            .collect();

        let mut write = self.remotes.write().await;

        for ip in &responses {
            if !self.current_remotes.contains(ip) {
                let a = VNode::new(ip.clone(), self.config.ratelimiter_port).await?;
                write.add(a.clone());
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn new(config: ReverseProxy) -> Self {
        let (rx, mut tx) = broadcast::channel(1);
        let obj = Self {
            remotes: Arc::new(RwLock::new(HashRingWrapper::default())),
            stop: Arc::new(rx),
            config,
            current_remotes: vec![],
        };

        let obj_clone = obj.clone();
        // Task to update the ratelimiters in the background
        tokio::spawn(async move {
            loop {
                debug!("refreshing");

                match obj_clone.get_ratelimiters().await {
                    Ok(_) => {
                        debug!("refreshed ratelimiting servers");
                    }
                    Err(err) => {
                        error!("refreshing ratelimiting servers failed {}", err);
                    }
                }

                let sleep = tokio::time::sleep(Duration::from_secs(5));
                tokio::pin!(sleep);
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
    pub fn ticket(
        &self,
        path: String,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static>> {
        let remotes = self.remotes.clone();
        Box::pin(
            async move {
                // Getting the node managing this path
                let mut node = remotes
                    .write()
                    .instrument(trace_span!("acquiring ring lock"))
                    .await
                    .get(&path)
                    .cloned()
                    .ok_or_else(|| {
                        anyhow!(
                            "did not compute ratelimit because no ratelimiter nodes are detected"
                        )
                    })?;

                // Initialize span for tracing (headers injection)
                let span = info_span!("remote request");
                let context = span.context();
                let mut request = Request::new(BucketSubmitTicketRequest { path });
                global::get_text_map_propagator(|propagator| {
                    propagator.inject_context(&context, &mut MetadataMap(request.metadata_mut()));
                });

                // Requesting
                node.submit_ticket(request)
                    .instrument(info_span!("waiting for ticket response"))
                    .await?;

                Ok(())
            }
            .instrument(Span::current()),
        )
    }

    pub fn submit_headers(
        &self,
        path: String,
        headers: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static>> {
        let remotes = self.remotes.clone();
        Box::pin(async move {
            let mut node = remotes
                .write()
                .instrument(trace_span!("acquiring ring lock"))
                .await
                .get(&path)
                .cloned()
                .ok_or_else(|| {
                    anyhow!("did not compute ratelimit because no ratelimiter nodes are detected")
                })?;

            let span = info_span!("remote request");
            let context = span.context();
            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_millis();
            // truncation is expected
            #[allow(clippy::cast_possible_truncation)]
            let mut request = Request::new(HeadersSubmitRequest {
                path,
                precise_time: time as u64,
                headers,
            });
            global::get_text_map_propagator(|propagator| {
                propagator.inject_context(&context, &mut MetadataMap(request.metadata_mut()));
            });

            node.submit_headers(request).await?;

            Ok(())
        })
    }
}
