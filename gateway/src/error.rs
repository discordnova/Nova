use common::error::NovaError;

#[derive(Debug)]
pub struct GatewayError(NovaError);

impl From<tokio_tungstenite::tungstenite::Error> for GatewayError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        GatewayError {
            0: NovaError {
                message: e.to_string(),
            },
        }
    }
}

impl From<String> for GatewayError {
    fn from(e: String) -> Self {
        GatewayError {
            0: NovaError {
                message: e,
            },
        }
    }
}
