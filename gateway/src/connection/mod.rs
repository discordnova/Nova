use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::http::Request};

use crate::{error::GatewayError, utils::get_gateway_url};

mod stream;
mod utils;

/// Underlying representation of a Discord event stream
/// that streams the Event payloads to the shard structure
pub struct Connection {
    /// The channel given by tokio_tungstenite that represents the websocket connection
    connection: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Connection {
    pub fn new() -> Self {
        Connection { connection: None }
    }

    pub async fn start(&mut self) -> Result<(), GatewayError> {
        let request = Request::builder()
            .uri(get_gateway_url(false, "json", 9))
            .body(())
            .unwrap();

        let connection_result = connect_async(request).await;
        // we connect outselves to the websocket server
        if let Err(err) = connection_result {
            Err(GatewayError::from(err))
        } else {
            self.connection = Some(connection_result.unwrap().0);
            Ok(())
        }
    }
}