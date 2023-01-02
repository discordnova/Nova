use std::net::ToSocketAddrs;

use futures_util::FutureExt;
use grpc::RLServer;
use leash::{ignite, AnyhowResultFuture, Component};
use proto::nova::ratelimit::ratelimiter::ratelimiter_server::RatelimiterServer;
use redis_global_local_bucket_ratelimiter::RedisGlobalLocalBucketRatelimiter;
use shared::{config::Settings, redis_crate::Client};
use tokio::sync::oneshot;
use tonic::transport::Server;

mod grpc;
mod redis_global_local_bucket_ratelimiter;

struct RatelimiterServerComponent {}
impl Component for RatelimiterServerComponent {
    type Config = ();
    const SERVICE_NAME: &'static str = "rest";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            // let config = Arc::new(settings.config);
            let redis: Client = settings.redis.into();
            let server = RLServer::new(RedisGlobalLocalBucketRatelimiter::new(redis.into()));

            Server::builder()
                .add_service(RatelimiterServer::new(server))
                .serve_with_shutdown(
                    "0.0.0.0:8080".to_socket_addrs().unwrap().next().unwrap(),
                    stop.map(|_| ()),
                )
                .await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}

ignite!(RatelimiterServerComponent);
