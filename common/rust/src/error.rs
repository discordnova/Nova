use std::fmt;

#[derive(Debug)]
pub struct NovaError {
    pub message: String,
}

impl fmt::Display for NovaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured wihind the nova system: {}", self.message) // user-facing output
    }
}
