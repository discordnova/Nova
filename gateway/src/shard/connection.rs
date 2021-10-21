use std::{cmp::min, convert::TryInto, time::Duration};

use crate::{connection::Connection, error::GatewayError, shard::state::SessionState};

use super::{state::ConnectionState, ConnectionWithState, Shard};
use common::{log::{debug, error, info}, types::{
        dispatch::Dispatch,
        user::User,
        ws::websocket::{BasePacket, WebsocketPacket},
    }};
use futures::StreamExt;
use tokio::{
    select,
    time::{interval_at, sleep, Instant},
};

impl Shard {
    pub async fn start(self: &mut Self) {
        let mut reconnects = 1;
        info!("Starting shard");

        while reconnects < self.config.max_reconnects {
            info!("Starting connection for shard");
            if let Err(e) = self._shard_task().await {
                error!("Gateway status: {:?}", e);
            }
            // when the shard got disconnected, the shard task ends
            reconnects += 1;

            // wait reconnects min(max(reconnects * reconnect_delay_growth_factor, reconnect_delay_minimum),reconnect_delay_maximum)
            if reconnects < self.config.max_reconnects {
                let time = min(
                    self.config.reconnect_delay_maximum,
                    self.config.reconnect_delay_minimum
                        * (((reconnects - 1) as f32) * self.config.reconnect_delay_growth_factor)
                            as usize,
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

    async fn _shard_task(&mut self) -> Result<(), GatewayError> {
        // create the new connection
        let mut connection = Connection::new();
        connection.start().await.unwrap();
        self.connection = Some(ConnectionWithState {
            conn: connection,
            state: ConnectionState::new(),
        });

        loop {
            if let Some(connection) = &mut self.connection {
                if let Some(timer) = &mut connection.state.interval {
                    select!(
                        payload = connection.conn.next() => {
                            match payload {
                                Some(data) => match data {
                                    Ok(message) => self._handle(message).await,
                                    Err(error) => {
                                        return Err(GatewayError::from(format!("An error occured while being connected to Discord: {:?}", error).to_string()));
                                    },
                                },
                                None => {
                                    return Err(GatewayError::from("Connection terminated".to_string()));
                                },
                            }
                        },
                        _ = timer.tick() => match self._do_heartbeat().await {
                            Ok(_) => {},
                            Err(error) => {
                                return Err(GatewayError::from(format!("An error occured while being connected to Discord: {:?}", error).to_string()));
                            },
                        }
                    )
                } else {
                    select!(
                        payload = connection.conn.next() => {
                            match payload {
                                Some(data) => match data {
                                    Ok(message) => self._handle(message).await,
                                    Err(error) => {
                                        return Err(GatewayError::from(format!("An error occured while being connected to Discord: {:?}", error).to_string()));
                                    },
                                },
                                None => {
                                    return Err(GatewayError::from("Connection terminated".to_string()));
                                },
                            }
                        }
                    )
                }
            }
        }
    }

    async fn _do_heartbeat(&mut self) -> Result<(), GatewayError> {
        info!("heartbeat sent");
        if let Some(conn) = &mut self.connection {
            if !conn.state.last_heartbeat_acknowledged {
                error!("we missed a hertbeat");
                Err(GatewayError::from(
                    "a hertbeat was dropped, we need to restart the connection".to_string(),
                ))
            } else {
                conn.state.last_heartbeat_acknowledged = false;
                conn.state.last_heartbeat_time = Instant::now();
                self._send_heartbeat().await
            }
        } else {
            unreachable!()
        }
    }

    async fn _handle(&mut self, message: BasePacket) {
        if let Some(seq) = &message.sequence {
            if let Some(state) = &mut self.state {
                state.sequence = seq.clone();
            }
        }
        let sequence = message.sequence.clone();
        debug!("packet type: {:?}", message.type_);

        let packet: Result<WebsocketPacket, serde_json::Error> = message.into();
        match packet {
            Ok(data) => match data {
                WebsocketPacket::Dispatch(msg) => {
                    self._dispatch(&msg, sequence).await;
                }
                // we need to reconnect to the gateway
                WebsocketPacket::Reconnect(msg) => {
                    info!("Gateway disconnect requested");
                    self._disconnect().await;
                }
                WebsocketPacket::InvalidSession(msg) => {
                    info!("invalid session");
                    if !msg {
                        info!("Session removed");
                        // reset the session data
                        self.state = None;
                        if let Err(e) = self._identify().await {
                            error!("Error while sending identify: {:?}", e);
                        }
                    }
                }
                WebsocketPacket::HeartbeatACK(_msg) => {
                    info!("Heartbeat ack received");
                    if let Some(conn) = &mut self.connection {
                        conn.state.last_heartbeat_acknowledged = true;
                        let latency = Instant::now() - conn.state.last_heartbeat_time;
                        info!("Latency updated {}ms", latency.as_millis());
                    }
                }
                WebsocketPacket::Hello(msg) => {
                    info!("Server hello received");
                    if let Some(conn) = &mut self.connection {
                        conn.state.interval = Some(interval_at(
                            Instant::now() + Duration::from_millis(msg.heartbeat_interval),
                            Duration::from_millis(msg.heartbeat_interval),
                        ));
                    }
    
                    if let Err(e) = self._identify().await {
                        error!("error while sending identify: {:?}", e);
                    }
                }
                _ => error!("unknown payload: {:?}", data),
            },
            Err(err) => error!("failed to deserialize payload: {:?}", err),
        }
    }

    async fn _dispatch(&mut self, dispatch: &Dispatch, sequence: Option<u64>) {
        match dispatch {
            Dispatch::Ready(ready) => {
                match &ready.user {
                    User::FullUser(user) => {
                        info!("Received gateway dispatch ready");
                        info!("Logged in as {}", user.username);
                    }

                    User::PartialUser(_) => todo!(),
                };
                self.state = Some(SessionState {
                    sequence: sequence.unwrap(),
                    session_id: ready.session_id.clone(),
                });
            }
            _ => {
                let name = format!(
                    "nova.cache.dispatch.{}",
                    dispatch.snake_case_name(),
                );
                debug!("event!: {:?}", name);
                if let Err(_) = self.nats.publish(
                    &name,
                    serde_json::to_string(dispatch).unwrap(),
                ) {
                    error!("failed to publish event!")
                }
            }
        }
    }
}
