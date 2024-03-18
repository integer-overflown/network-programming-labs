use std::fmt::{Display, Formatter};
use std::{io, string};

#[derive(Debug)]
pub enum ConnectionError {
    FailedToReadInput(io::Error),
    InvalidMessage(string::FromUtf8Error),
}

impl From<io::Error> for ConnectionError {
    fn from(value: io::Error) -> Self {
        Self::FailedToReadInput(value)
    }
}

impl From<string::FromUtf8Error> for ConnectionError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::InvalidMessage(value)
    }
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::FailedToReadInput(_) => write!(f, "Cannot read input buffer"),
            ConnectionError::InvalidMessage(_) => {
                write!(f, "Cannot parse incoming message as UTF-8")
            }
        }
    }
}

impl std::error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConnectionError::FailedToReadInput(e) => Some(e),
            ConnectionError::InvalidMessage(e) => Some(e),
        }
    }
}
