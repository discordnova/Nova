use super::{Connection, state::ConnectionState};

impl Connection {
    /// Returns the current state of the connection.
    pub fn state(&self) -> ConnectionState {
        return self.state.clone();
    }
}