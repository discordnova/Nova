mod client;

use common::config::Settings;

#[tokio::main]
async fn main() {    
    let settings: Settings<()> = Settings::new("gateway").unwrap();
}
