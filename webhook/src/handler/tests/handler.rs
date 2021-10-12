fn generate_keypair() -> (
    String,
    [u8; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize],
) {
    use libsodium_sys::crypto_sign_ed25519_keypair;
    let pk_s: String;

    let mut pk = [0; libsodium_sys::crypto_sign_ed25519_PUBLICKEYBYTES as usize];
    let mut sk = [0; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize];

    let pk_p = pk.as_mut_ptr();
    let sk_p = sk.as_mut_ptr();

    // generate keypair
    unsafe {
        if crypto_sign_ed25519_keypair(pk_p, sk_p) < 0 {
            panic!("keypair generation failed!");
        }
    };

    pk_s = hex::encode(pk);
    return (pk_s, sk);
}

fn sign_message(
    msg: Vec<u8>,
    sk: [u8; libsodium_sys::crypto_sign_ed25519_SECRETKEYBYTES as usize],
) -> String {
    use libc::c_ulonglong;
    use libsodium_sys::crypto_sign_ed25519_detached;

    let len = msg.len();
    let mut signature_len: c_ulonglong = 0;
    let mut str = [0; 64];
    unsafe {
        crypto_sign_ed25519_detached(
            str.as_mut_ptr(),
            &mut signature_len,
            msg.as_ptr(),
            len as u64,
            sk.as_ptr(),
        );
    };

    return hex::encode(str);
}

#[tokio::test]
async fn respond_to_pings_and_deny_invalid() {
    use crate::start;
    use common::config::test_init;
    use common::config::Settings;
    use common::log::info;
    use common::testcontainers::images::generic::GenericImage;
    use common::testcontainers::Docker;
    use hyper::{Body, Method, Request};
    use libsodium_sys::sodium_init;
    use serde_json::json;
    use std::time::Duration;

    test_init();

    unsafe {
        if sodium_init() < 0 {
            panic!("libsodium init error!");
        }
    }

    let (private_key, secret_key) = generate_keypair();
    let ping = json!({ "type": 1 }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, secret_key);

    // start nats
    let docker = common::testcontainers::clients::Cli::default();
    let image = GenericImage::new("nats");
    let node = docker.run(image);
    node.start();
    let port = node.get_host_port(4222).unwrap();

    let settings: Settings<crate::config::Config> = common::config::Settings {
        config: crate::config::Config {
            server: crate::config::ServerSettings {
                port: 5003,
                address: "0.0.0.0".to_string(),
            },
            discord: crate::config::Discord {
                public_key: private_key,
                client_id: 0,
            },
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
    };

    let nats: common::nats_crate::Connection = settings.nats.clone().into();
    // start the server
    tokio::task::spawn(start(settings));
    tokio::time::sleep(Duration::from_secs(1)).await;

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

    // setup nats mock listener
    let sub = nats.subscribe("nova.cache.dispatch.interaction").unwrap();

    let ping = json!({ "type": 0 }).to_string();
    let timestamp = "my datetime :)";
    let signature_data = [timestamp.as_bytes().to_vec(), ping.as_bytes().to_vec()].concat();
    let signature = sign_message(signature_data, secret_key);

    // we must timeout
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature.clone())
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 500);

     sub.with_handler(move |msg| {
        info!("Received {}", &msg);
        msg.respond("ok :)").unwrap();
        Ok(())
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5003/")
        .header("X-Signature-Ed25519", signature.clone())
        .header("X-Signature-Timestamp", timestamp)
        .body(Body::from(ping.clone()))
        .expect("request builder");
    let client = hyper::client::Client::new();
    let result = client.request(req).await.unwrap();
    assert!(result.status() == 200);
}
