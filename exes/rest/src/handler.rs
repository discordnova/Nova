use anyhow::bail;
use http::{
    header::{AUTHORIZATION, CONNECTION, HOST, TRANSFER_ENCODING, UPGRADE},
    HeaderValue, Method as HttpMethod, Request, Response, Uri,
};
use hyper::{client::HttpConnector, Body, Client};
use hyper_rustls::HttpsConnector;
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryFrom,
    hash::{Hash, Hasher},
    str::FromStr,
    sync::Arc,
};
use tracing::{debug_span, error, info_span, instrument, Instrument};
use twilight_http_ratelimiting::{Method, Path};

use crate::ratelimit_client::RemoteRatelimiter;

/// Normalizes the path
fn normalize_path(request_path: &str) -> (&str, &str) {
    if let Some(trimmed_path) = request_path.strip_prefix("/api") {
        if let Some(maybe_api_version) = trimmed_path.split('/').nth(1) {
            if let Some(version_number) = maybe_api_version.strip_prefix('v') {
                if version_number.parse::<u8>().is_ok() {
                    let len = "/api/v".len() + version_number.len();
                    return (&request_path[..len], &request_path[len..]);
                };
            };
        }

        ("/api", trimmed_path)
    } else {
        ("/api", request_path)
    }
}

#[instrument]
pub async fn handle_request(
    client: Client<HttpsConnector<HttpConnector>, Body>,
    ratelimiter: Arc<RemoteRatelimiter>,
    token: String,
    mut request: Request<Body>,
) -> Result<Response<Body>, anyhow::Error> {
    let (hash, uri_string) = {
        let method = match *request.method() {
            HttpMethod::DELETE => Method::Delete,
            HttpMethod::GET => Method::Get,
            HttpMethod::PATCH => Method::Patch,
            HttpMethod::POST => Method::Post,
            HttpMethod::PUT => Method::Put,
            _ => {
                error!("Unsupported HTTP method in request, {}", request.method());
                bail!("unsupported method");
            }
        };

        let request_path = request.uri().path();
        let (api_path, trimmed_path) = normalize_path(request_path);

        let mut uri_string = format!("http://127.0.0.1:9999{api_path}{trimmed_path}");
        if let Some(query) = request.uri().query() {
            uri_string.push('?');
            uri_string.push_str(query);
        }

        let mut hash = DefaultHasher::new();
        match Path::try_from((method, trimmed_path)) {
            Ok(path) => path,
            Err(e) => {
                error!(
                    "Failed to parse path for {:?} {}: {:?}",
                    method, trimmed_path, e
                );
                bail!("failed to parse");
            }
        }
        .hash(&mut hash);

        (hash.finish().to_string(), uri_string)
    };
    // waits for the request to be authorized
    ratelimiter
        .ticket(hash.clone())
        .instrument(debug_span!("ticket validation request"))
        .await?;

    request
        .headers_mut()
        .insert(HOST, HeaderValue::from_static("discord.com"));

    // Remove forbidden HTTP/2 headers
    // https://datatracker.ietf.org/doc/html/rfc7540#section-8.1.2.2
    request.headers_mut().remove(CONNECTION);
    request.headers_mut().remove("keep-alive");
    request.headers_mut().remove("proxy-connection");
    request.headers_mut().remove(TRANSFER_ENCODING);
    request.headers_mut().remove(UPGRADE);

    if let Some(auth) = request.headers_mut().get_mut(AUTHORIZATION) {
        if auth
            .to_str()
            .expect("Failed to check header")
            .starts_with("Bot")
        {
            *auth = HeaderValue::from_str(&format!("Bot {token}"))?;
        }
    } else {
        request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bot {token}"))?,
        );
    }

    let uri = match Uri::from_str(&uri_string) {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to create URI for requesting Discord API: {:?}", e);
            bail!("failed to create uri");
        }
    };
    *request.uri_mut() = uri;
    let span = debug_span!("upstream request to discord");
    let resp = match client.request(request).instrument(span).await {
        Ok(response) => response,
        Err(e) => {
            error!("Error when requesting the Discord API: {:?}", e);
            bail!("failed to request the discord api");
        }
    };

    let headers = resp
        .headers()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_str().map(std::string::ToString::to_string)))
        .filter(|f| f.1.is_ok())
        .map(|f| (f.0, f.1.expect("errors should be filtered")))
        .collect();

    let _submit_headers = ratelimiter
        .submit_headers(hash, headers)
        .instrument(info_span!("submitting headers"))
        .await;

    Ok(resp)
}
