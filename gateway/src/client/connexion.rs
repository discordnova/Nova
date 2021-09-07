use crate::client::payloads::{message::OpCodes, payloads::{Hello, Identify, IdentifyProprerties}};
use super::{
    payloads::message::MessageBase,
    state::{Stage, State},
    utils::get_gateway_url,
};
use flate2::write::ZlibDecoder;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use std::{str::from_utf8, time::Duration};
use tokio::{net::TcpStream, select, time::Instant};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{self, Message, handshake::client::Request}};

#[derive(Debug)]
pub enum CloseReason {
    ConnexionAlredyOpen,
    ConnexionEnded,
    ErrorEncountered(&'static str),
    ConnexionError(tungstenite::Error),
}

pub struct Config {
    pub token: String,
    pub compress: bool,
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
    config: Config,
    zlib: ZlibDecoder<Vec<u8>>,
    connexion: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    nats: Option<async_nats::Connection>,
}

impl Connexion {
    /// Creates a new instance of a discord websocket connexion using the options
    /// this is used internally by the shard struct to initialize a single
    /// websocket connexion. This instance is not initialized by default.
    /// a websocket connexion like this can be re-used multiple times
    /// to allow reconnexion mechanisms.
    pub async fn new(config: Config) -> Self {
        Connexion {
            state: State::default(),
            connexion: None,
            config,
            zlib: ZlibDecoder::<Vec<u8>>::new(vec![]),
            nats: None,
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
            let request = Request::builder()
                .header("User-Agant", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36")
                .uri(get_gateway_url(false, "json", 9))
                .body(())
                .unwrap();

            let connexion_result = connect_async(request).await;
            // we connect outselves to the websocket server
            if let Err(err) = connexion_result {
                return CloseReason::ConnexionError(err);
            }
            self.connexion = Some(connexion_result.unwrap().0);
            self.nats = Some(async_nats::connect("localhost:4222").await.unwrap());
            // this is the loop that will maintain the whole connexion
            loop {
                if let Some(connexion) = &mut self.connexion {
                    // if we do not have a hello message received yet, then we do not use the heartbeat interval
                    // and we just wait for messages to arrive
                    if self.state.stage == Stage::Unknown {
                        let msg = connexion.next().await;
                        if let HandleResult::Error(reason) = self._handle_message(&msg).await {
                            return reason;
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
                    return CloseReason::ConnexionEnded;
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

                    _ => HandleResult::Error(CloseReason::ErrorEncountered(
                        "unsupported message type encountered",
                    )),
                },
                Err(_error) => HandleResult::Error(CloseReason::ErrorEncountered(
                    "error while reading a message",
                )),
            }
        } else {
            HandleResult::Error(CloseReason::ErrorEncountered(
                "error while reading a message",
            ))
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
                info!("dispatch message received: {:?}", t);
                let topic = format!("nova.gateway.{}", t);
                self.nats.as_ref().unwrap().publish(
                    &topic,
                    &serde_json::to_vec(&message.d).unwrap(),
                ).await.unwrap();
            },
            OpCodes::PresenceUpdate => todo!(),
            OpCodes::VoiceStateUpdate => todo!(),
            OpCodes::Reconnect => todo!(),
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
                    self._send(&MessageBase {
                        t: None,
                        op: OpCodes::Identify,
                        s: None,
                        d: serde_json::to_value(&Identify{
                            token: self.config.token.clone(),
                            intents: 1 << 0 |
                                1 << 1 |
                                1 << 2 |
                                1 << 3 |
                                1 << 4 |
                                1 << 5 |
                                1 << 6 |
                                1 << 7 |
                                1 << 8 |
                                1 << 9 | 
                                1 << 10 |
                                1 << 11 |
                                1 << 12 |
                                1 << 13 |
                                1 << 14,
                            properties: IdentifyProprerties {
                                os: "Linux".into(),
                                browser: "Nova".into(),
                                device: "Linux".into(),
                            }
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
            self._terminate_websocket(CloseReason::ErrorEncountered(
                "the server did not acknowledged the last heartbeat",
            ))
            .await;
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
                    self._terminate_websocket(CloseReason::ErrorEncountered(
                        "failed to write to the socket",
                    ))
                    .await;
                }
            } else {
                self._terminate_websocket(CloseReason::ErrorEncountered(
                    "failed to serialize the message",
                ))
                .await;
            }
        }
    }
}
