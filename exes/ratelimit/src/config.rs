use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

fn default_listening_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8090))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub listening_adress: SocketAddr,
}
impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            listening_adress: default_listening_address(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RatelimitServerConfig {
    pub server: ServerSettings,
}
