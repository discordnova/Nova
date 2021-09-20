use tokio::time::{Instant, Interval};

/// This struct represents the state of a session
#[derive(Clone, Debug)]
pub struct SessionState {
    pub sequence: u64,
    pub session_id: String,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            sequence: Default::default(),
            session_id: Default::default(),
        }
    }
}

/// This struct represents the state of a connection
#[derive(Debug)]
pub struct ConnectionState {
    pub last_heartbeat_acknowledged: bool,
    pub last_heartbeat_time: Instant,
    pub interval: Option<Interval>,
    
}
impl ConnectionState {
    pub fn new() -> Self {
        Self {
            last_heartbeat_acknowledged: true,
            last_heartbeat_time: Instant::now(),
            interval: None,
        }
    }
}