use crate::error::SocketError;
use std::net::UdpSocket;
use tracing::{debug, info};

const MSG_HEADER_LENGTH: usize = std::mem::size_of::<u16>();

fn read_message_header(socket: &UdpSocket) -> Result<usize, SocketError> {
    let mut buf = [0; MSG_HEADER_LENGTH];
    let (num_read, peer_addr) = socket.peek_from(&mut buf)?;

    info!("Got datagram from {}", peer_addr);

    if num_read < buf.len() {
        return Err(SocketError::MessageTooShort {
            actual: num_read,
            minimal: buf.len(),
        });
    }

    Ok(u16::from_be_bytes(buf) as usize)
}

fn read_message(
    socket: &UdpSocket,
    expected_len: usize,
    out_buf: &mut Vec<u8>,
) -> Result<(), SocketError> {
    debug!("Reading message of size {expected_len}");

    let total_message_size = expected_len + MSG_HEADER_LENGTH;
    out_buf.resize(total_message_size, 0);

    let (num_read, _) = socket.recv_from(out_buf)?;

    if num_read < total_message_size {
        return Err(SocketError::UnexpectedEof {
            expected_len: total_message_size,
            actual: num_read,
        });
    }

    Ok(())
}

pub fn receive_message(socket: &UdpSocket) -> Result<(), SocketError> {
    let message_size = read_message_header(socket)?;
    let mut buf = Vec::new();

    read_message(socket, message_size, &mut buf)?;

    let message = String::from_utf8_lossy(&buf[MSG_HEADER_LENGTH..]);

    info!("Got message of size {message_size}: {message}");

    Ok(())
}
