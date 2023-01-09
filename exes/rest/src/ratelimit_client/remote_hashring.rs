use core::fmt::Debug;
use opentelemetry::propagation::Injector;
use proto::nova::ratelimit::ratelimiter::ratelimiter_client::RatelimiterClient;
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct VNode {
    address: String,

    client: RatelimiterClient<Channel>,
}

impl Deref for VNode {
    type Target = RatelimiterClient<Channel>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for VNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl Hash for VNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

pub struct MetadataMap<'a>(pub &'a mut tonic::metadata::MetadataMap);

impl<'a> Injector for MetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

impl VNode {
    pub async fn new(address: String, port: u16) -> Result<Self, tonic::transport::Error> {
        let client =
            RatelimiterClient::connect(format!("http://{}:{}", address.clone(), port)).await?;

        Ok(VNode { client, address })
    }
}

unsafe impl Send for VNode {}

#[repr(transparent)]
#[derive(Default)]
pub struct HashRingWrapper(hashring::HashRing<VNode>);

impl Deref for HashRingWrapper {
    type Target = hashring::HashRing<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HashRingWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for HashRingWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HashRing").finish()
    }
}
