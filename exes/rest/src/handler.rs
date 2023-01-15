use anyhow::{bail, Context};
use futures_util::FutureExt;
use http::{
    header::{AUTHORIZATION, CONNECTION, HOST, TRANSFER_ENCODING, UPGRADE},
    HeaderValue, Method as HttpMethod, Request, Response, Uri,
};
use hyper::{client::HttpConnector, Body, Client};
use hyper_rustls::HttpsConnector;
use opentelemetry::{
    global,
    metrics::{Counter, Histogram},
    Context as OpenTelemetryContext, KeyValue,
};
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryFrom,
    hash::{Hash, Hasher},
    str::FromStr,
    sync::Arc,
    time::SystemTime,
};
use tracing::{debug_span, error, info_span, log::trace, Instrument};
use twilight_http_ratelimiting::{Method, Path};

use crate::{config::ReverseProxy, ratelimit_client::RemoteRatelimiter};
use lazy_static::lazy_static;

lazy_static! {
    static ref METER_NAME: &'static str = "";
    static ref REQUESTS: Counter<u64> = {
        global::meter(&METER_NAME)
            .u64_counter("rest.http_requests_total")
            .with_description("Amount of requests processed by the rest reverse proxy")
            .init()
    };
    static ref UPSTREAM_CALLS: Counter<u64> = {
        global::meter(&METER_NAME)
            .u64_counter("rest.upstream_http_requests_total")
            .with_description("Amount of requests sent to discord")
            .init()
    };
    static ref TICKET_CALLS: Counter<u64> = {
        global::meter(&METER_NAME)
            .u64_counter("rest.ticket_http_requests_total")
            .with_description("Amount of requests sent to the ratelimiter")
            .init()
    };
    static ref HEADERS_SUBMIT_CALLS: Counter<u64> = {
        global::meter(&METER_NAME)
            .u64_counter("rest.header_submit_http_requests_total")
            .with_description("Amount of requests sent to the ratelimiter")
            .init()
    };
    static ref UPSTREAM_TIMES: Histogram<f64> = {
        global::meter(&METER_NAME)
            .f64_histogram("rest.upstream_http_request_duration_seconds")
            .with_description("Time took to request discord")
            .init()
    };
    static ref TICKET_TIMES: Histogram<f64> = {
        global::meter(&METER_NAME)
            .f64_histogram("rest.ticket_http_request_duration_seconds")
            .with_description("Time took to get a ticket from the ratelimiter")
            .init()
    };
    static ref HEADERS_SUBMIT_TIMES: Histogram<f64> = {
        global::meter(&METER_NAME)
            .f64_histogram("rest.header_submit_http_request_duration_seconds")
            .with_description("Time took to get a ticket from the ratelimiter")
            .init()
    };
}

/// Normalizes the path
#[inline]
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

#[inline]
#[allow(clippy::too_many_lines)]
pub async fn handle_request(
    client: Client<HttpsConnector<HttpConnector>, Body>,
    ratelimiter: Arc<RemoteRatelimiter>,
    config: ReverseProxy,
    token: String,
    mut request: Request<Body>,
) -> Result<Response<Body>, anyhow::Error> {
    let cx = OpenTelemetryContext::current();

    let (bucket, uri_string) = {
        let method = match *request.method() {
            HttpMethod::DELETE => Method::Delete,
            HttpMethod::GET => Method::Get,
            HttpMethod::PATCH => Method::Patch,
            HttpMethod::POST => Method::Post,
            HttpMethod::PUT => Method::Put,
            _ => {
                error!(method =? request.method(), "unsupported HTTP method in request");
                bail!("unsupported method");
            }
        };
        let request_path = request.uri().path();
        let (api_path, trimmed_path) = normalize_path(request_path);
        trace!("normalized path to {trimmed_path}");

        let mut uri_string = format!(
            "{}{api_path}{trimmed_path}",
            config.upstream.expect("no upstream")
        );
        if let Some(query) = request.uri().query() {
            uri_string.push('?');
            uri_string.push_str(query);
        }

        trace!("full request uri is {uri_string}");

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
        let bucket = hash.finish().to_string();
        trace!("Request bucket is {}", bucket);

        (bucket, uri_string)
    };

    REQUESTS.add(&cx, 1, &[KeyValue::new("bucket", bucket.clone())]);

    let ticket_start = SystemTime::now();
    TICKET_CALLS.add(&cx, 1, &[KeyValue::new("bucket", bucket.clone())]);
    // waits for the request to be authorized
    match ratelimiter
        .ticket(bucket.clone())
        .instrument(debug_span!("ticket validation request"))
        .then(|v| async {
            TICKET_TIMES.record(
                &cx,
                ticket_start.elapsed()?.as_secs_f64(),
                &[KeyValue::new("bucket", bucket.clone())],
            );
            v
        })
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error when requesting the ratelimiter: {:?}", e);
            bail!("failed to request the ratelimiter");
        }
    }

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
    let upstream_start = SystemTime::now();
    UPSTREAM_CALLS.add(&cx, 1, &[KeyValue::new("bucket", bucket.clone())]);
    let resp = match client
        .request(request)
        .instrument(span)
        .then(|v| async {
            UPSTREAM_TIMES.record(
                &cx,
                upstream_start.elapsed()?.as_secs_f64(),
                &[KeyValue::new("bucket", bucket.clone())],
            );
            v.context("")
        })
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Error when requesting the Discord API: {:?}", e);
            bail!("failed to request the discord api");
        }
    };

    let headers = resp
        .headers()
        .into_iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().map(std::string::ToString::to_string),
            )
        })
        .filter(|f| f.1.is_ok())
        .map(|f| (f.0, f.1.expect("errors should be filtered")))
        .collect();

    HEADERS_SUBMIT_CALLS.add(&cx, 1, &[KeyValue::new("bucket", bucket.clone())]);
    let _submit_headers = ratelimiter
        .submit_headers(bucket.clone(), headers)
        .instrument(info_span!("submitting headers"))
        .then(|v| async {
            HEADERS_SUBMIT_TIMES.record(
                &cx,
                upstream_start.elapsed()?.as_secs_f64(),
                &[KeyValue::new("bucket", bucket.clone())],
            );
            v
        })
        .await;

    Ok(resp)
}
