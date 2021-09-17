mod client;

use client::connection::Connection;
use common::config::Settings;
use futures::StreamExt;
use log::info;
use serde_json::Value;

use crate::client::payloads::{dispatch::Dispatch, gateway::{FullMessage, Message, OpCodes}, payloads::identify::{Identify, IdentifyProprerties}};

#[tokio::main]
async fn main() {
    let settings: Settings<Value> = Settings::new("gateway").unwrap();

    let mut conn = Connection::new();
    conn.start().await.unwrap();

    loop {
        if let Some(val) = conn.next().await {
            let data = val.as_ref().unwrap();
            match data {
                Message::Dispatch(dispatch) => {
                    match &dispatch.data {
                        Dispatch::Ready(_ready) => {
                            
                        },
                    }
                },
                Message::Reconnect(_) => todo!(),
                Message::InvalidSession(_) => todo!(),
                Message::Hello(_hello) => {
                    info!("Server said hello! {:?}", _hello);
                },
                Message::HeartbeatACK(_) => todo!(),
            }

        } else {
            break;
        }
    }
}
