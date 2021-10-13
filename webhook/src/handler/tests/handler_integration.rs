use std::time::Duration;

use crate::{
    config::Config,
    handler::tests::utils::{generate_keypair, sign_message},
    start,
};
use common::{config::test_init, nats_crate::Connection, testcontainers::images::generic::WaitFor};
use common::{
    config::Settings,
    log::info,
    testcontainers::{clients::Cli, images::generic::GenericImage, Container, Docker},
};
use hyper::{Body, Method, Request};
use lazy_static::{__Deref, lazy_static};
use serde_json::json;

#[cfg(all(unix, target_arch = "x86_64"))]
const fn nats_image<'a>() -> &'a str {
    return "amd64/nats";
}

#[cfg(all(unix, target_arch = "aarch64"))]
const fn nats_image<'a>() -> &'a str {
    return "arm64v8/nats";
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
const fn nats_image<'a>() -> &'a str {
    return "winamd64/nats";
}

lazy_static! {
    static ref DOCKER: Cli = Cli::default();

    static ref NATS_CONTAINER: Container<'static, Cli, GenericImage> = {
        test_init();

        let image: GenericImage = GenericImage::new(nats_image())
            .with_wait_for(WaitFor::message_on_stderr("Server is ready"));
        
        let container = DOCKER.run(image);
        container.start();
        container.get_host_port(4222).unwrap();
        container
    };

    static ref KEYPAIR: (String, [u8; 64]) = {
        generate_keypair()
    };

    static ref SETTINGS: Settings<Config> = {
        let port = NATS_CONTAINER.get_host_port(4222).unwrap();
        common::config::Settings {
            config: crate::config::Config {
                server: crate::config::ServerSettings {
                    port: 5003,
                    address: "0.0.0.0".to_string(),
                },
                discord: crate::config::Discord {
                    public_key: KEYPAIR.0.clone(),
                    client_id: 0,
                },
            },
            redis: common::redis::RedisConfiguration {
                url: "".to_string(),
            },
            monitoring: common::monitoring::MonitoringConfiguration {
                enabled: false,
                address: None,
                port: None,
            },
            nats: common::nats::NatsConfiguration {
                client_cert: None,
                root_cert: None,
                jetstream_api_prefix: None,
                max_reconnects: None,
                reconnect_buffer_size: None,
                tls: None,
                client_name: None,
                tls_required: None,
                host: format!("localhost:{}", port),
            },
        }
    };

    static ref TASK: () = {
        std::thread::spawn(|| {
            let r = tokio::runtime::Runtime::new().unwrap();
            r.spawn(async { start(SETTINGS.clone()).await });
            loop {}
        });
        std::thread::sleep(Duration::from_secs(1));
    };
}

#[tokio::test]
async fn respond_to_pings() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    let ping = json!({ "type": 1 }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, KEYPAIR.1);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();

    assert!(result.status() == 200);
}

#[tokio::test]
async fn deny_invalid_signatures() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    let ping = json!({ "type": 1 }).to_string();
    let timestamp = "my datetime :)";

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", "inva&lid signature :)")
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 401);
}

#[tokio::test]
async fn response_500_when_no_nats_response() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    let ping = json!({ "type": 0 }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, KEYPAIR.1);

    // we must timeout
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");

    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 500);
}

#[tokio::test]
async fn respond_from_nats_response() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    let nats: Connection = SETTINGS.clone().nats.into();
    let sub = nats.subscribe("nova.cache.dispatch.interaction").unwrap();
    let ping = json!({ "type": 0 }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, KEYPAIR.1);

    sub.with_handler(move |msg| {
        info!("Received {}", &msg);
        msg.respond("ok :)").unwrap();
        Ok(())
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 200);
}

#[tokio::test]
async fn response_400_when_invalid_json_body() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    let ping = "{".to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, KEYPAIR.1);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 400);
}

#[tokio::test]
async fn response_400_when_invalid_utf8_body() {
    let _ = NATS_CONTAINER.deref();
    let _ = TASK.deref();
    // invalid 2 octet sequence
    let ping = vec![0xc3, 0x28];

    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.to_vec()].concat();
    let signature = sign_message(signature_data, KEYPAIR.1);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 400);
}
