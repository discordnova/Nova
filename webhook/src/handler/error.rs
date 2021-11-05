use hyper::{header::ToStrError, Body, Response, StatusCode};

pub struct WebhookError {
    pub code: StatusCode,
    pub message: String,
}

impl WebhookError {
    pub fn new(code: StatusCode, message: &str) -> WebhookError {
        WebhookError {
            code,
            message: message.to_string(),
        }
    }
}

impl Into<Response<Body>> for WebhookError {
    fn into(self) -> Response<Body> {
        Response::builder()
            .status(self.code)
            .body(self.message.into())
            .unwrap()
    }
}

impl From<hyper::Error> for WebhookError {
    fn from(_: hyper::Error) -> Self {
        WebhookError::new(StatusCode::BAD_REQUEST, "invalid request")
    }
}

impl From<ToStrError> for WebhookError {
    fn from(_: ToStrError) -> Self {
        WebhookError::new(StatusCode::BAD_REQUEST, "invalid request")
    }
}
