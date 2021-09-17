use super::Shard;
use crate::client::connection::Connection;
use log::info;

impl Shard {
    async fn start(self: &mut Self) {
        let mut should_exit = false;

        while !should_exit {
            info!("Starting connection for shard");
            // create the new connection
            self.connection = Some(Connection::new());
            should_exit = true;
        }
    }
}
