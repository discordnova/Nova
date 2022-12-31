use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use log::{error, info};
use prometheus::{Encoder, TextEncoder};
use serde::Deserialize;
use std::net::ToSocketAddrs;

#[derive(Clone, Debug, Deserialize)]
/// Options for the monitoring service
pub struct MonitoringConfiguration {
    pub enabled: bool,
    pub address: Option<String>,
    pub port: Option<i32>,
}

/// Handler for the hyper http server
async fn serve_metrics(_request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metrics = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    Ok(response)
}

/// Starts a monitoring server on the requested port
pub fn start_monitoring(configuration: &MonitoringConfiguration) {
    let config = configuration.clone();
    tokio::task::spawn(async move {
        if config.enabled {
            let address = format!(
                "{}:{}",
                config
                    .address
                    .expect("a listening address must be specified for the metrics server"),
                config
                    .port
                    .expect("a listening port must be specified for the metrics server")
            );
            info!("Starting monitoring server on {}", address);

            let listen_address = address.to_socket_addrs().unwrap().next().unwrap();
            let server = Server::bind(&listen_address).serve(make_service_fn(|_| async {
                Ok::<_, hyper::Error>(service_fn(serve_metrics))
            }));

            if let Err(e) = server.await {
                error!("failed to start the monitoring server {}", e);
            }
        }
    });
}
