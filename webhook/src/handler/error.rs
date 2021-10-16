use hyper::{Body, Error, Response, StatusCode};

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
