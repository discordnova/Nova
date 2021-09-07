use client::traits::message_handler::MessageHandler;
extern crate serde_json;

mod client;

struct Handler {}
impl MessageHandler for Handler {}

#[tokio::main]
async fn main() {    
    pretty_env_logger::init();
    for _ in 0..1 {
        tokio::spawn(async move {
            let con = client::connexion::Connexion::new().await;
            con.start().await;
        }).await.unwrap();
    }
}