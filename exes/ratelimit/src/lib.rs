use buckets::redis_lock::RedisLock;
use config::RatelimitServerConfig;
use grpc::RLServer;
use leash::{AnyhowResultFuture, Component};
use proto::nova::ratelimit::ratelimiter::ratelimiter_server::RatelimiterServer;
use redis::aio::MultiplexedConnection;
use shared::config::Settings;
use std::future::Future;
use std::{pin::Pin};
use tokio::sync::oneshot;
use tonic::transport::Server;

mod config;
mod grpc;
mod buckets;

pub struct RatelimiterServerComponent {}
impl Component for RatelimiterServerComponent {
    type Config = RatelimitServerConfig;
    const SERVICE_NAME: &'static str = "ratelimiter";

    fn start(
        &self,
        settings: Settings<Self::Config>,
        stop: oneshot::Receiver<()>,
    ) -> AnyhowResultFuture<()> {
        Box::pin(async move {
            let listening_address = settings.server.listening_adress;
            let redis = Into::<
                Pin<Box<dyn Future<Output = anyhow::Result<MultiplexedConnection>> + Send>>,
            >::into(settings.redis)
            .await?;

            let server = RLServer::new(RedisLock::new(redis));

            Server::builder()
                .add_service(RatelimiterServer::new(server))
                .serve_with_shutdown(listening_address, async move {
                    let _ = stop.await;
                })
                .await?;

            Ok(())
        })
    }

    fn new() -> Self {
        Self {}
    }
}
