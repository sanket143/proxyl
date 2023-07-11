use std::{error, fmt, io};

#[derive(Debug)]
pub struct ProxylError {
    message: String,
}

impl ProxylError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl error::Error for ProxylError {}

impl fmt::Display for ProxylError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for ProxylError {
    fn from(err: io::Error) -> Self {
        ProxylError::new(err)
    }
}
