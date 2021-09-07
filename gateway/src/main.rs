use std::env;

use client::{connexion::Config, traits::message_handler::MessageHandler};
extern crate serde_json;
extern crate serde_repr;

mod client;

struct Handler {}
impl MessageHandler for Handler {}

#[tokio::main]
async fn main() {    
    pretty_env_logger::init();
    for _ in 0..1 {
        tokio::spawn(async move {
            let con = client::connexion::Connexion::new(Config {
                token: env::var("DISCORD_TOKEN").expect("A valid token is required").into(),
                compress: false,
            }).await;
            println!("{:?}", con.start().await);
        }).await.unwrap();
    }
}