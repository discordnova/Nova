use shared::config::Settings;
use log::info;

use crate::config::CacheConfiguration;

mod config;


fn main() {
    let settings: Settings<CacheConfiguration> = Settings::new("cache").unwrap();
    info!("loaded configuration: {:?}", settings);

    
}
