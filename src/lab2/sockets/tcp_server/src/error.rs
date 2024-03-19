use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ConnectionError {
    IoError(io::Error),
}

impl From<io::Error> for ConnectionError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::IoError(_) => write!(f, "Cannot read input buffer"),
        }
    }
}

impl std::error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConnectionError::IoError(e) => Some(e),
        }
    }
}
