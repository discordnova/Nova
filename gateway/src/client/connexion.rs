use crate::client::payloads::{message::OpCodes, payloads::{Hello, Identify, IdentifyProprerties}};
use super::{
    payloads::message::MessageBase,
    state::{Stage, State},
    utils::get_gateway_url,
};
use futures_util::{SinkExt, StreamExt};
use log::{error, info, trace, warn};
use std::{str::from_utf8, time::Duration};
use tokio::{net::TcpStream, select, time::Instant};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{self, Message, handshake::client::Request}};
use crate::client::structs::ClientConfig;
use tokio::sync::mpsc;

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

pub struct Connexion {
    state: State,
    config: ClientConfig,
    connexion: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    terminate: Option<mpsc::Sender<CloseReason>>,
}

impl Connexion {
    pub fn new(config: ClientConfig) -> Self {
        Connexion {
            state: State::default(),
            connexion: None,
            config,
            terminate: None
        }
    }

    /// Terminate the connexion and the "start" method related to it.
    async fn _terminate_websocket(&mut self, message: &CloseReason) {
        if let Some(connexion) = &mut self.connexion {
            if let Err(err) = connexion.close(None).await {
                error!("failed to close socket {}", err);
            } else {
                info!("closed the socket: {:?}", message);
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
            let request = Request::builder()
                .uri(get_gateway_url(false, "json", 9))
                .body(())
                .unwrap();

            let connexion_result = connect_async(request).await;
            // we connect outselves to the websocket server
            if let Err(err) = connexion_result {
                return CloseReason::ConnexionError(err);
            }
            self.connexion = Some(connexion_result.unwrap().0);

            let (tx, mut rx) = mpsc::channel::<CloseReason>(1);
            self.terminate = Some(tx);

            // this is the loop that will maintain the whole connexion
            loop {
                if let Some(connexion) = &mut self.connexion {
                    // if we do not have a hello message received yet, then we do not use the heartbeat interval
                    // and we just wait for messages to arrive
                    if self.state.stage == Stage::Unknown {
                        select! {
                            msg = connexion.next() => self._handle_message(&msg).await,
                            Some(reason) = rx.recv() => {
                                // gateway termination requested
                                self._terminate_websocket(&reason);
                                return reason
                            }
                        }
                    } else {
                        let timer = self.state.interval.as_mut().unwrap().tick();
                        select! {
                            msg = connexion.next() => self._handle_message(&msg).await,
                            _ = timer => self._do_heartbeat().await,
                            Some(reason) = rx.recv() => {
                                // gateway termination requested
                                self._terminate_websocket(&reason);
                                return reason
                            }
                        }
                    }
                } else {
                    return CloseReason::ConnexionEnded;
                }
            }
        }
    }

    async fn _handle_message(
        &mut self,
        data: &Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
    ) {
        if let Some(message) = data {
            match message {
                Ok(message) => match message {
                    Message::Text(text) => {
                        self._handle_discord_message(&text).await;
                    }
                    Message::Binary(message) => {
                        self._handle_discord_message(from_utf8(message).unwrap())
                            .await;
                    }
                    Message::Close(code) => {
                        error!("discord connexion closed: {:?}", code);
                        self.terminate.as_ref().unwrap().send(CloseReason::ConnexionEnded).await.unwrap();
                    }

                    _ => self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                        "unsupported message type encountered",
                    )).await.unwrap(),
                },
                Err(_error) => self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                    "error while reading a message",
                )).await.unwrap(),
            }
        } else {
            self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                "error while reading a message",
            )).await.unwrap()
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
            OpCodes::Dispatch => {
                let t = message.t.unwrap();
                trace!("dispatch message received: {:?}", t);
            },
            OpCodes::PresenceUpdate => {
                println!("presence update message received: {:?}", message.d);
            },
            OpCodes::VoiceStateUpdate => {
                println!("voice update");
            }
            OpCodes::Reconnect => {
                println!("reconnect {:?}", message.d);
            },
            OpCodes::InvalidSession => {
                println!("invalid session: {:?}", message.d);
            },
            OpCodes::Hello => {
                if let Ok(hello) = serde_json::from_value::<Hello>(message.d) {
                    info!("server sent hello {:?}", hello);
                    info!("heartbeating every {}ms", hello.heartbeat_interval);
                    self.state.interval = Some(tokio::time::interval_at(
                        Instant::now() + Duration::from_millis(hello.heartbeat_interval),
                        Duration::from_millis(hello.heartbeat_interval),
                    ));
                    self.state.stage = Stage::Initialized;
                    let mut shard: Option<[i64; 2]> = None;
                    if let Some(sharding) = &self.config.shard {
                        shard = Some([sharding.current_shard.clone(), sharding.total_shards.clone()]);
                        info!("shard information: {:?}", shard);
                    }
                    self._send(&MessageBase {
                        t: None,
                        op: OpCodes::Identify,
                        s: None,
                        d: serde_json::to_value(&Identify{
                            token: self.config.token.clone(),
                            intents: self.config.intents.clone().bits(),
                            properties: IdentifyProprerties {
                                os: "Linux".into(),
                                browser: "Nova".into(),
                                device: "Linux".into(),
                            },
                            shard: shard,
                        }).unwrap(),
                    }).await;
                    // do login
                    // todo: session logic
                }
            }
            OpCodes::HeartbeatACK => {
                info!(
                    "heartbeat acknowledged after {}ms",
                    (std::time::Instant::now() - self.state.last_heartbeat_time).as_millis()
                );
                self.state.last_heartbeat_acknowledged = true;
            }
            _ => {} // invalid payloads
        }
    }

    async fn _do_heartbeat(&mut self) {
        if !self.state.last_heartbeat_acknowledged {
            self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                "the server did not acknowledged the last heartbeat",
            )).await.unwrap();
            return;
        }
        self.state.last_heartbeat_acknowledged = false;

        info!("sending heartbeat");
        self._send(&MessageBase {
            t: None,
            d: serde_json::to_value(self.state.sequence).unwrap(),
            s: None,
            op: OpCodes::Heartbeat,
        })
        .await;
        self.state.last_heartbeat_time = std::time::Instant::now();
    }

    async fn _send(&mut self, data: &MessageBase) {
        if let Some(connexion) = &mut self.connexion {
            if let Ok(json) = serde_json::to_vec(data) {
                if let Err(error) = connexion.send(Message::Binary(json)).await {
                    error!("failed to write to socket: {}", error);
                    self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                        "failed to write to the socket",
                    ))
                    .await.unwrap();
                }
            } else {
                self.terminate.as_ref().unwrap().send(CloseReason::ErrorEncountered(
                    "failed to serialize the message",
                ))
                .await.unwrap();
            }
        }
    }
}
