use std::time::Instant;
use tokio::time::Interval;

#[derive(PartialEq)]
pub enum Stage {
    Unknown,
    Initialized,
    LoggedIn,
}

pub struct State {
    pub stage: Stage,
    pub sequence: i64,
    pub last_heartbeat_acknowledged: bool,
    pub last_heartbeat_time: Instant,
    pub interval: Option<Interval>,
}

impl State {
    pub fn default() -> Self {
        State {
            sequence: 0,
            interval: None,
            stage: Stage::Unknown,
            last_heartbeat_acknowledged: true,
            last_heartbeat_time: std::time::Instant::now(),
        }
    }
}
