use nats::{Connection, Options};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfigurationClientCert {
    pub cert: String,
    pub key: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfigurationTls {
    pub mtu: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfiguration {
    pub client_cert: Option<NatsConfigurationClientCert>,
    pub root_cert: Option<Vec<String>>,
    pub jetstream_api_prefix: Option<String>,
    pub max_reconnects: Option<usize>,
    pub reconnect_buffer_size: Option<usize>,
    pub tls: Option<NatsConfigurationTls>,
    pub client_name: Option<String>,
    pub tls_required: Option<bool>,
    pub host: String,
}

// Allows the configuration to directly create a nats connection
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
