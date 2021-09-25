use nats::{Options, Connection};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct NatsConfigurationClientCert {
    cert: String,
    key: String
}
#[derive(Clone, Debug, Deserialize)]
struct NatsConfigurationTls {
    mtu: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfiguration {
    client_cert: Option<NatsConfigurationClientCert>,
    root_cert: Option<Vec<String>>,
    jetstream_api_prefix: Option<String>,
    max_reconnects: Option<usize>,
    reconnect_buffer_size: Option<usize>,
    tls: Option<NatsConfigurationTls>,
    client_name: Option<String>,
    tls_required: Option<bool>,
    host: String,
}

/// 
impl Into<Connection> for NatsConfiguration {
    fn into(self) -> Connection {
        let mut options = Options::new();
        
        if let Some(client_cert) = self.client_cert {
            options = options.client_cert(client_cert.cert, client_cert.key);
        }

        if let Some(root_certs) = self.root_cert {
            for root_cert in root_certs {
                options = options.add_root_certificate(root_cert);
            }
        }

        if let Some(jetstream_api_prefix) = self.jetstream_api_prefix {
            options = options.jetstream_api_prefix(jetstream_api_prefix)
        }

        options = options.max_reconnects(self.max_reconnects);
        options = options.no_echo();
        options = options.reconnect_buffer_size(self.reconnect_buffer_size.unwrap_or(64 * 1024));
        options = options.tls_required(self.tls_required.unwrap_or(false));
        options = options.with_name(&self.client_name.unwrap_or("Nova".to_string()));


        if let Some(tls) = self.tls {
            let mut config = nats::rustls::ClientConfig::new();
            config.set_mtu(&tls.mtu);
            // todo: more options?
            options = options.tls_client_config(config);
        }

        options.connect(&self.host).unwrap()
    }
}
