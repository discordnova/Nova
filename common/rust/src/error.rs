use std::fmt;

#[derive(Debug)]
pub struct NovaError {
    pub message: String,
}

impl fmt::Display for NovaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred within the nova system: {}", self.message) // user-facing output
    }
}

impl From<&str> for NovaError {
    fn from(message: &str) -> Self {
        NovaError { message: message.to_string() }
    }
}