use std::fmt::{Display, Formatter};
use std::{error, io};

#[derive(Debug)]
pub enum SocketError {
    ReadFailed(io::Error),
    UnexpectedEof { expected_len: usize, actual: usize },
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketError::ReadFailed(_) => write!(f, "Failed to read from socket"),
            SocketError::UnexpectedEof {
                expected_len,
                actual,
            } => {
                write!(
                    f,
                    "Unexpected EOF when reading message: expected {} bytes, got {} bytes",
                    expected_len, actual
                )
            }
        }
    }
}

impl From<io::Error> for SocketError {
    fn from(value: io::Error) -> Self {
        Self::ReadFailed(value)
    }
}

impl error::Error for SocketError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            SocketError::ReadFailed(e) => Some(e),
            _ => None,
        }
    }
}
