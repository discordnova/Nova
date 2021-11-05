use std::time::Duration;

use ctor;
use hyper::{Body, Method, Request, StatusCode};
use lazy_static::lazy_static;
use serde_json::json;
use ed25519_dalek::Keypair;

use common::{
    config::test_init,
    nats_crate::Connection,
    testcontainers::{Image, images::generic::WaitFor},
};
use common::{
    config::Settings,
    log::info,
    testcontainers::{clients::Cli, Container, Docker, images::generic::GenericImage},
};

use crate::{
    config::Config,
    handler::tests::utils::{generate_keypair, sign_message},
    start,
};

const fn nats_image<'a>() -> &'a str {
    #[cfg(all(unix, target_arch = "x86_64"))]
    return "amd64/nats";
    #[cfg(all(unix, target_arch = "aarch64"))]
    return "arm64v8/nats";
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    return "winamd64/nats";
}

static mut NATS: Option<Container<Cli, GenericImage>> = None;
static mut SETTINGS: Option<Settings<Config>> = None;

lazy_static! {
    static ref TEST_KEYPAIR: Keypair = generate_keypair();
    static ref DOCKER: Cli = Cli::default();
}

#[ctor::ctor]
unsafe fn init() {
    test_init();
    let image = GenericImage::new(nats_image())
        .with_wait_for(WaitFor::message_on_stderr("Server is ready"));

    let container = DOCKER.run(image);
    container.start();
    container.image().wait_until_ready(&container);
    container.get_host_port(4222).unwrap();

    let port = container.get_host_port(4222).unwrap();
    NATS = Some(container);
    SETTINGS = Some(common::config::Settings {
        config: crate::config::Config {
            server: crate::config::ServerSettings {
                port: 5003,
                address: "0.0.0.0".to_string(),
            },
            discord: crate::config::Discord {
                public_key: hex::encode(TEST_KEYPAIR.public.clone()),
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
    });
    let settings = (&mut SETTINGS).as_ref().unwrap();

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(start(settings.clone()));
    });
    std::thread::sleep(Duration::from_secs(3));
}

#[ctor::dtor]
unsafe fn destroy() {
    let nats = (&mut NATS).as_ref().unwrap();
    nats.stop();
}

#[tokio::test]
async fn respond_to_pings() {
    let ping = json!({ "type": 1, "id": "0", "application_id": "0", "token": "random token", "version": 1, "channel_id": "123" }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, &TEST_KEYPAIR);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");

    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();

    assert_eq!(result.status(), StatusCode::OK);
}

#[tokio::test]
async fn deny_invalid_signatures() {
    let ping = json!({ "type": 1, "id": "0", "application_id": "0", "token": "random token", "version": 1, "channel_id": "123" }).to_string();
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
    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn response_500_when_no_nats_response() {
    let ping = json!({
        "type": 2,
        "id": "0",
        "application_id": "0",
        "token": "random token",
        "version": 1,
        "channel_id": "123",
        "data": {
            "id": "0",
            "name": "command"
        }
    }).to_string();

    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, &TEST_KEYPAIR);

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
    assert_eq!(result.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn respond_from_nats_response() {
    let nats: Connection;
    unsafe {
        nats = SETTINGS.clone().unwrap().nats.into();
    }
    let sub = nats.subscribe("nova.cache.dispatch.interaction").unwrap();
    let ping = json!({
        "type": 2,
        "id": "0",
        "application_id": "0",
        "token": "random token",
        "version": 1,
        "channel_id": "123",
        "data": {
            "id": "0",
            "name": "command"
        }
    }).to_string();
    
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, &TEST_KEYPAIR);

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
    assert_eq!(result.status(), StatusCode::OK);
}

#[tokio::test]
async fn response_400_when_invalid_json_body() {
    let ping = "{".to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, &TEST_KEYPAIR);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn response_400_when_invalid_utf8_body() {
    // invalid 2 octet sequence
    let ping = vec![0xc3, 0x28];

    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.to_vec()].concat();
    let signature = sign_message(signature_data, &TEST_KEYPAIR);

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature)
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}
