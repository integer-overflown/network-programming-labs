use std::fmt::{Display, Formatter};
use std::{io, string};

#[derive(Debug)]
pub enum SocketError {
    FailedToReadInput(io::Error),
    InvalidMessage(string::FromUtf8Error),
}

impl From<io::Error> for SocketError {
    fn from(value: io::Error) -> Self {
        Self::FailedToReadInput(value)
    }
}

impl From<string::FromUtf8Error> for SocketError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::InvalidMessage(value)
    }
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketError::FailedToReadInput(_) => write!(f, "Cannot read input buffer"),
            SocketError::InvalidMessage(_) => {
                write!(f, "Cannot parse incoming message as UTF-8")
            }
        }
    }
}

impl std::error::Error for SocketError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SocketError::FailedToReadInput(e) => Some(e),
            SocketError::InvalidMessage(e) => Some(e),
        }
    }
}
