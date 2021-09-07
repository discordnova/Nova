use crate::client::payloads::{message::OpCodes, payloads::Hello};

use super::{
    payloads::message::MessageBase,
    state::{Stage, State},
    utils::get_gateway_url,
};
use futures_util::{
    SinkExt, StreamExt,
};
use log::{error, info, warn};
use std::{str::from_utf8, time::Duration};
use tokio::{
    net::TcpStream,
    select,
    time::{Instant},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{self, Message}};

#[derive(Debug)]
pub enum CloseReason {
    ConnexionAlredyOpen,
    ConnexionEnded,
    ErrorEncountered(&'static str),
    ConnexionError(tungstenite::Error),
}

pub enum HandleResult {
    Success,
    Error(CloseReason),
}

/// This struct represents a single connexion to the gateway,
/// it does not have any retry logic or reconnexion mechanism,
/// everything is handled in the Shard struct.
/// The purpose of this struct is to handle the encoding,
/// compression and other gateway-transport related stuff.
/// All the messages are send through another struct implementing
/// the MessageHandler trait.
pub struct Connexion {
    state: State,
    connexion: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Connexion {
    /// Creates a new instance of a discord websocket connexion using the options
    /// this is used internally by the shard struct to initialize a single
    /// websocket connexion. This instance is not initialized by default.
    /// a websocket connexion like this can be re-used multiple times
    /// to allow reconnexion mechanisms.
    pub async fn new() -> Self {
        Connexion {
            state: State::default(),
            connexion: None,
        }
    }

    /// Terminate the connexion and the "start" method related to it.
    async fn _terminate_websocket(&mut self, message: CloseReason) {
        if let Some(connexion) = &mut self.connexion {
            if let Err(err) = connexion.close(None).await {
                error!("failed to close socket {}", err);
            } else {
                info!("closed the socket: {:?}", message)
            }
        } else {
            warn!("a termination request was sent without a connexion openned")
        }
    }

    /// Initialize a connexion to the gateway
    /// returns if a connexion is already present
    pub async fn start(mut self) -> CloseReason {
        if let Some(_) = self.connexion {
            CloseReason::ConnexionAlredyOpen
        } else {
            // we reset the state before starting the connection
            self.state = State::default();

            let connexion_result = connect_async(get_gateway_url(false, "json", 9)).await;
            // we connect outselves to the websocket server
            if let Err(err) = connexion_result {
                return CloseReason::ConnexionError(err)
            }
            self.connexion = Some(connexion_result.unwrap().0);

            // this is the loop that will maintain the whole connexion
            loop {
                if let Some(connexion) = &mut self.connexion {
                    // if we do not have a hello message received yet, then we do not use the heartbeat interval
                    // and we just wait for messages to arrive
                    if self.state.stage == Stage::Unknown {
                        let msg = connexion.next().await;
                        if let HandleResult::Error(reason) = self._handle_message(&msg).await {
                            return reason
                        }
                    } else {
                        let timer = self.state.interval.as_mut().unwrap().tick();
                        select! {
                            msg = connexion.next() => {
                                if let HandleResult::Error(reason) = self._handle_message(&msg).await {
                                    return reason
                                }
                            },
                            _ = timer => self._do_heartbeat().await
                        }
                    }
                } else {
                    return CloseReason::ConnexionEnded
                }
            }
        }
    }

    async fn _handle_message(
        &mut self,
        data: &Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
    ) -> HandleResult {
        if let Some(message) = data {
            match message {
                Ok(message) => match message {
                    Message::Text(text) => {
                        self._handle_discord_message(&text).await;
                        HandleResult::Success
                    }
                    Message::Binary(message) => {
                        self._handle_discord_message(from_utf8(message).unwrap())
                            .await;
                        HandleResult::Success
                    }
                    Message::Close(_) => {
                        error!("discord connexion closed");
                        HandleResult::Error(CloseReason::ConnexionEnded)
                    }

                    _ => {
                        HandleResult::Error(CloseReason::ErrorEncountered("unsupported message type encountered"))
                    }
                },
                Err(_error) => {
                    HandleResult::Error(CloseReason::ErrorEncountered("error while reading a message"))
                }
            }
        } else {
            HandleResult::Error(CloseReason::ErrorEncountered("error while reading a message"))
        }
    }

    async fn _handle_discord_message(&mut self, raw_message: &str) {
        let a: Result<MessageBase, serde_json::Error> = serde_json::from_str(raw_message);
        let message = a.unwrap();

        // handles the state
        if let Some(index) = message.s {
            self.state.sequence = index;
        }

        match message.op {
            OpCodes::Dispatch => todo!(),
            OpCodes::Heartbeat => todo!(),
            OpCodes::Identify => todo!(),
            OpCodes::PresenceUpdate => todo!(),
            OpCodes::VoiceStateUpdate => todo!(),
            OpCodes::Resume => todo!(),
            OpCodes::Reconnect => todo!(),
            OpCodes::RequestGuildMembers => todo!(),
            OpCodes::InvalidSession => todo!(),
            OpCodes::Hello => {
                if let Ok(hello) = serde_json::from_value::<Hello>(message.d) {
                    info!("server sent hello {:?}", hello);
                    info!("heartbeating every {}ms", hello.heartbeat_interval);
                    self.state.interval = Some(tokio::time::interval_at(
                        Instant::now() + Duration::from_millis(hello.heartbeat_interval),
                        Duration::from_millis(hello.heartbeat_interval),
                    ));
                    self.state.stage = Stage::Initialized;
                }
            }
            OpCodes::HeartbeatACK => {
                info!(
                    "heartbeat acknowledged after {}ms",
                    (std::time::Instant::now() - self.state.last_heartbeat_time).as_millis()
                );
                self.state.last_heartbeat_acknowledged = true;
            }
        }
    }

    async fn _do_heartbeat(&mut self) {
        if !self.state.last_heartbeat_acknowledged {
            self._terminate_websocket(CloseReason::ErrorEncountered("the server did not acknowledged the last heartbeat")).await;
            return;
        }
        self.state.last_heartbeat_acknowledged = false;

        info!("sending heartbeat");
        self._send(
            serde_json::to_vec(&MessageBase {
                t: None,
                d: serde_json::to_value(self.state.sequence).unwrap(),
                s: None,
                op: OpCodes::Heartbeat,
            })
            .unwrap(),
        )
        .await;
        self.state.last_heartbeat_time = std::time::Instant::now();
    }

    async fn _send(&mut self, data: Vec<u8>) {
        if let Some(connexion) = &mut self.connexion {
            if let Err(error) = connexion.send(Message::Binary(data)).await {
                error!("failed to write to socket: {}", error);
                self._terminate_websocket(CloseReason::ErrorEncountered("failed to write to the socket")).await;
            }
        }
    }
}
