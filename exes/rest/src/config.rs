use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use serde::Deserialize;

fn default_listening_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8090))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    #[serde(default = "default_listening_address")]
    pub listening_adress: SocketAddr
}
impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            listening_adress: default_listening_address(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Discord {
    pub token: String
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ReverseProxyConfig {
    pub server: ServerSettings,
    pub discord: Discord,
}
