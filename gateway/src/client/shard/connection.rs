use super::Shard;
use crate::client::connection::Connection;
use log::info;

impl Shard {
    pub async fn start(self: &mut Self) {
        let mut should_exit = false;

        while !should_exit {
            info!("Starting connection for shard");
            // create the new connection
            let mut connection = Connection::new();
            connection.start().await.unwrap();
            self.connection = Some(connection);
            should_exit = true;
        }
    }
}
