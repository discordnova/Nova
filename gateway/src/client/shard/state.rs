use std::time::Instant;

/// This struct represents the state of a session
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
#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub sequence: u64,
    pub last_heartbeat_acknowledged: bool,
    pub last_heartbeat_time: Instant,
    
}
impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            sequence: 0,
            last_heartbeat_acknowledged: true,
            last_heartbeat_time: Instant::now(),
        }
    }
}

impl ConnectionState {}