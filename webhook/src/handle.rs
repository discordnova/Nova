use hyper::service::Service;
use hyper::{
    body::{to_bytes, Bytes},
    HeaderMap,
};
use hyper::{Body, Method, Request, Response, StatusCode};
use libsodium_sys::crypto_sign_ed25519_verify_detached;
use log::info;
use std::future;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use super::utils::Settings;

static NOT_FOUND: &str = "
<h1>Nova Webhook Gateway</h1>
<p>Invalid request</p>
";

pub fn validate_signature(b64_public_key: &str, data: &Bytes, b64_signature: &str) -> bool {
    // First, we need to check if the signature & private key is valid base64.
    let signature_result = base64::decode(b64_signature);
    let public_key_result = base64::decode(b64_public_key);

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

fn get_signature<'b>(headers: &'b HeaderMap) -> Option<(&'b str, &'b str)> {
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
                let sig_headers = get_signature(&headers);
                if sig_headers.is_some() {
                    let (signature, _timestamp) = sig_headers.unwrap();
                    let data = to_bytes(req.into_body()).await.unwrap();
                    info!("data: {}", public_key);
                    if validate_signature(public_key.as_str(), &data, signature) {
                        return Ok(Response::new("signature verified!".into()));
                    }
                    return Ok(Response::new("signature verification failed.".into()));
                }
                return Ok(Response::new("no signature specified.".into()));
            });
        } else {
            return Box::pin(async {
                let mut response = Response::new(NOT_FOUND.into());
                let status = response.status_mut();
                *status = StatusCode::BAD_REQUEST;
                Ok(response)
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
    use hyper::body::Bytes;

    #[test]
    fn validate_signature_test() {
        let signature = "VD7DVH1X+d2x7ExcNlA+vyiP/aPaPVEHZMmknCq7V2kO+XTGPRdHcb3SSB3hBmlm9Xq77BKj7Bcbn24jc4NwAg==";
        let public_key = "7v4MJEc3N8sgNSMuO065HCBvChRoQWjzUD99gxYFjW8=";
        let content = "message de test incroyable";
        assert!(validate_signature(
            public_key,
            &Bytes::from(content),
            signature
        ))
    }

    #[test]
    fn validate_signature_reverse_test() {
        let signature = "VD7DVH1X+d2x7ExcNlA+vyiP/aPaPVEHZMmknCq7V2kO+XTGPRdHcb3SSB3hBmlm9Xq77BKj7Bcbn24jc4NwAg==";
        let public_key = "wCnuoYQ3KSyHxirsNOfRvU44/mEm8/fERt5jddxmYEQ=";
        let content = "ceci est un test qui ne fonctionnera pas!";
        assert!(!validate_signature(
            public_key,
            &Bytes::from(content),
            signature
        ))
    }

    #[test]
    fn invalid_base64() {
        let signature = "zzz";
        let public_key = "zzz";
        let content = "ceci est un test qui ne fonctionnera pas!";
        assert!(!validate_signature(
            public_key,
            &Bytes::from(content),
            signature
        ))
    }
}
