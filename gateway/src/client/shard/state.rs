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
