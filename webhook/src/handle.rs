use hyper::service::Service;
use hyper::{body::to_bytes, HeaderMap};
use hyper::{Body, Method, Request, Response, StatusCode};
use libsodium_sys::crypto_sign_ed25519_verify_detached;
use log::info;
use serde_json::Value;
use std::future;
use std::future::Future;
use std::pin::Pin;
use std::str::from_utf8;
use std::task::{Context, Poll};
use serde::{Deserialize, Serialize};

use super::utils::Settings;

pub fn validate_signature(b64_public_key: &str, data: &Vec<u8>, b64_signature: &str) -> bool {
    // First, we need to check if the signature & private key is valid base64.
    let signature_result = hex::decode(b64_signature);
    let public_key_result = hex::decode(b64_public_key);

    if signature_result.is_ok() && public_key_result.is_ok() {
        // Since we now have the signatures in u8 vectors. We will initialize all the
        // parameters for the ffi call to sodium.
        let signature_pointer = signature_result.unwrap();
        let private_key_pointer = public_key_result.unwrap();

        let data_pointer = data.as_ptr();
        let data_len = data.len() as u64;

        // A ffi call is considered unsafe by the Rust compiler
        // we assume all the parameters are correct for the call
        unsafe {
            // If the signature is valid, sodium will return 0
            return crypto_sign_ed25519_verify_detached(
                signature_pointer.as_ptr(),
                data_pointer,
                data_len,
                private_key_pointer.as_ptr(),
            ) == 0;
        }
    }
    false
}

fn get_signature(headers: &HeaderMap) -> Option<(&str, &str)> {
    let signature = headers.get("X-Signature-Ed25519");
    let timestamp = headers.get("X-Signature-Timestamp");

    if signature.is_some() && timestamp.is_some() {
        return Some((
            signature.unwrap().to_str().unwrap(),
            timestamp.unwrap().to_str().unwrap(),
        ));
    }
    None
}

pub struct HandlerService {
    pub config: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
    #[serde(rename = "type")]
    t: i32
}

impl Service<Request<Body>> for HandlerService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if req.method() == Method::POST {
            let public_key = self.config.discord.public_key.clone();
            return Box::pin(async move {
                let headers = req.headers().clone();
                if let Some((signature, timestamp)) = get_signature(&headers) {
                    if let Ok(data) = to_bytes(req.into_body()).await {
                        let contatenated_data = [timestamp.as_bytes().to_vec(), data.to_vec()].concat();

                        if validate_signature(public_key.as_str(), &contatenated_data, signature) {
                            let data: Value = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
                            let t = data.get("type").unwrap().as_i64().unwrap();

                            if t == 1 {
                                info!("success!");

                                return Ok(Response::builder().header("Content-Type", "application/json").body(serde_json::to_string(&Ping {
                                    t: 1
                                }).unwrap().into()).unwrap());
                                
                            } else {
                                Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body("invalid operation".into()).unwrap())
                            }
                        } else {
                            Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body("signature verification failed".into()).unwrap())
                        }
                    } else {
                        Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body("failed to read body".into()).unwrap())
                    }
                } else {
                    Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body("no signature specified".into()).unwrap())
                }
            });
        } else {
            return Box::pin(async {
                Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body("bad method".into()).unwrap())
            });
        }
    }
}

pub struct MakeSvc {
    pub settings: Settings,
}

impl<T> Service<T> for MakeSvc {
    type Response = HandlerService;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ready(Ok(HandlerService {
            config: self.settings.clone(),
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::handle::validate_signature;

    #[test]
    fn validate_signature_test() {
        let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
        let public_key = "eefe0c24473737cb2035232e3b4eb91c206f0a14684168f3503f7d8316058d6f";
        let content = "message de test incroyable".as_bytes().to_vec();
        assert!(validate_signature(public_key, &content, signature))
    }

    #[test]
    fn validate_signature_reverse_test() {
        let signature = "543ec3547d57f9ddb1ec4c5c36503ebf288ffda3da3d510764c9a49c2abb57690ef974c63d174771bdd2481de1066966f57abbec12a3ec171b9f6e2373837002";
        let public_key = "c029eea18437292c87c62aec34e7d1bd4e38fe6126f3f7c446de6375dc666044";
        let content = "ceci est un test qui ne fonctionnera pas!"
            .as_bytes()
            .to_vec();
        assert!(!validate_signature(public_key, &content, signature))
    }

    #[test]
    fn invalid_hex() {
        let signature = "zzz";
        let public_key = "zzz";
        let content = "ceci est un test qui ne fonctionnera pas!"
            .as_bytes()
            .to_vec();
        assert!(!validate_signature(public_key, &content, signature))
    }
}
