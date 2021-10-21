use crate::{error::GatewayError};

use super::Connection;
use futures::{FutureExt, Sink, SinkExt, Stream, StreamExt};
use common::{log::info, types::ws::websocket::WebsocketPacket};
use serde::Serialize;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_tungstenite::tungstenite::Message;

/// Implementation of the Stream trait for the Connection
impl Stream for Connection {
    type Item = Result<WebsocketPacket, GatewayError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // first, when a poll is called, we check if the connection is still open
        if let Some(conn) = &mut self.connection {
            // we need to wait poll the message using the tokio_tungstenite stream
            let message = conn.poll_next_unpin(cx);

            match message {
                Poll::Ready(packet) => {
                    // if data is available, we can continue
                    match packet {
                        Some(result) => match result {
                            Ok(message) => {
                                match Box::pin(self._handle_message(&message)).poll_unpin(cx) {
                                    Poll::Ready(data) => match data {
                                        Ok(d) => Poll::Ready(Some(Ok(d))),
                                        Err(e) => Poll::Ready(Some(Err(e))),
                                    },
                                    // unknown behaviour?
                                    Poll::Pending => unreachable!(),
                                }
                            }
                            Err(e) => Poll::Ready(Some(Err(GatewayError::from(e)))),
                        },
                        // if no message is available, we return none, it's the end of the stream
                        None => {
                            info!("tokio_tungstenite stream finished successfully");
                            let _ = Box::pin(conn.close(None)).poll_unpin(cx);
                            self.connection = None;
                            Poll::Ready(None)
                        }
                    }
                }
                // if the message is pending, we return the same result
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        }
    }
}

/// Implementation of the Sink trait for the Connection
impl Sink<WebsocketPacket> for Connection {
    type Error = tokio_tungstenite::tungstenite::Error;

    #[allow(dead_code)]
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(conn) = &mut self.connection {
            // a connection exists, we can send data
            conn.poll_ready_unpin(cx)
        } else {
            Poll::Pending
        }
    }

    #[allow(dead_code)]
    fn start_send(mut self: Pin<&mut Self>, item: WebsocketPacket) -> Result<(), Self::Error> {
        if let Some(conn) = &mut self.connection {
            let message = serde_json::to_string(&item);
            conn.start_send_unpin(Message::Text(message.unwrap()))
                .unwrap();
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(conn) = &mut self.connection {
            conn.poll_flush_unpin(cx)
        } else {
            Poll::Pending
        }
    }

    #[allow(dead_code)]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(conn) = &mut self.connection {
            conn.poll_close_unpin(cx)
        } else {
            Poll::Pending
        }
    }
}
