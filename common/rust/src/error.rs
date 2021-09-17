use std::fmt;

pub struct NovaError {
    pub message: String,
}

impl fmt::Display for NovaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured wihind the nova system: {}", self.message) // user-facing output
    }
}

impl fmt::Debug for NovaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

