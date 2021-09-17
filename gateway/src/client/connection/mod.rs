use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

mod stream;
mod state;
mod actions;

/// Underlying representation of a Discord event stream
/// that streams the Event payloads to the shard structure
pub struct Connection {
    /// The channel given by tokio_tungstenite that represents the websocket connection
    connection: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    /// The state of the connection
    state: state::ConnectionState,
}

impl Connection {
    pub fn new() -> Self {
        Connection {
            connection: None,
            state: state::ConnectionState::default()
        }
    }
}