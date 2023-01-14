use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use ed25519_dalek::PublicKey;
use serde::{Deserialize, Deserializer};

fn default_listening_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8091))
}

#[derive(Debug, Deserialize, Clone, Copy)]
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

fn deserialize_pk<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    let public_key = PublicKey::from_bytes(&hex::decode(str).unwrap()).unwrap();
    Ok(public_key)
}

#[derive(Debug, Deserialize, Clone, Default, Copy)]
pub struct Discord {
    #[serde(deserialize_with = "deserialize_pk")]
    pub public_key: PublicKey,
}

#[derive(Debug, Deserialize, Clone, Default, Copy)]
pub struct Webhook {
    pub server: ServerSettings,
    pub discord: Discord,
}
