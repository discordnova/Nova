use std::{
    cmp::{max, min},
    convert::TryInto,
    time::Duration,
};

use crate::{
    connection::Connection,
    payloads::{
        dispatch::Dispatch,
        gateway::{BaseMessage, Message},
    },
    shard::state::SessionState,
};

use super::{state::ConnectionState, ConnectionWithState, Shard};
use futures::StreamExt;
use log::{error, info};
use tokio::{select, time::sleep};

impl Shard {
    pub async fn start(self: &mut Self) {
        let mut reconnects = 1;
        info!("Starting shard");

        while reconnects < self.config.max_reconnects {
            info!("Starting connection for shard");
            self._shard_task().await;
            // when the shard got disconnected, the shard task ends
            reconnects += 1;

            // wait reconnects min(max(reconnects * reconnect_delay_growth_factor, reconnect_delay_minimum),reconnect_delay_maximum)
            if reconnects < self.config.max_reconnects {
                let time = min(
                    self.config.reconnect_delay_maximum,
                    self.config.reconnect_delay_minimum * (((reconnects - 1) as f32) * self.config.reconnect_delay_growth_factor) as usize,
                );
                info!(
                    "The shard got disconnected, waiting for reconnect ({}ms)",
                    time
                );
                sleep(Duration::from_millis(time.try_into().unwrap())).await;
            }
        }
        info!(
            "The shard got disconnected too many times and reached the maximum {}",
            self.config.max_reconnects
        );
    }

    async fn _shard_task(&mut self) {
        // create the new connection
        let mut connection = Connection::new();
        connection.start().await.unwrap();
        self.connection = Some(ConnectionWithState {
            conn: connection,
            state: ConnectionState::default(),
        });
        loop {
            if let Some(connection) = &mut self.connection {
                select!(
                    payload = connection.conn.next() => {
                        match payload {
                            Some(data) => match data {
                                Ok(message) => self._handle(&message).await,
                                Err(error) => {
                                    error!("An error occured while being connected to Discord: {:?}", error);
                                    return;
                                },
                            },
                            None => {
                                info!("Connection terminated");
                                return;
                            },
                        }
                    }
                )
            }
        }
    }

    fn _util_set_seq(&mut self, seq: Option<u64>) {
        if let Some(seq) = seq {
            if let Some(state) = &mut self.state {
                state.sequence = seq;
            }
        }
    }

    async fn _handle(&mut self, message: &Message) {
        match message {
            Message::Dispatch(msg) => {
                self._util_set_seq(msg.sequence);
                self._dispatch(&msg).await;
            }
            // we need to reconnect to the gateway
            Message::Reconnect(msg) => {
                self._util_set_seq(msg.sequence);
                info!("Gateway disconnect requested");
                self._disconnect().await;
            }
            Message::InvalidSession(msg) => {
                self._util_set_seq(msg.sequence);
                info!("invalid session");
                let data = msg.data;
                if !data {
                    info!("Session removed");
                    // reset the session data
                    self.state = None;
                    if let Err(e) = self._identify().await {
                        error!("Error while sending identify: {:?}", e);
                    }
                }
            }
            Message::HeartbeatACK(msg) => {
                self._util_set_seq(msg.sequence);
                info!("Heartbeat ack received");
            }
            Message::Hello(msg) => {
                self._util_set_seq(msg.sequence);
                info!("Server hello received");
                if let Err(e) = self._identify().await {
                    error!("error while sending identify: {:?}", e);
                }
            },
        }
    }

    async fn _dispatch(&mut self, dispatch: &BaseMessage<Dispatch>) {
        match &dispatch.data {
            Dispatch::Ready(ready) => {
                info!("Received gateway dispatch ready");
                info!(
                    "Logged in as {}",
                    ready.user.get("username").unwrap().to_string()
                );
                self.state = Some(SessionState {
                    sequence: dispatch.sequence.unwrap(),
                    session_id: ready.session_id.clone(),
                });
            }
            Dispatch::Other(data) => {
                
            }
        }
    }
}
