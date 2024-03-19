use crate::error::SocketError;
use server_utils::message;
use std::net::UdpSocket;
use task::Config;
use tracing::debug;

pub fn handle_datagram(config: &Config, socket: &UdpSocket) -> Result<(), SocketError> {
    let mut buf = [0; message::MESSAGE_SIZE];
    let (num_read, address) = socket.recv_from(&mut buf)?;

    debug!("Received datagram of size {num_read} from {address}");

    if num_read < buf.len() {
        return Err(SocketError::UnexpectedEof {
            actual: num_read,
            expected_len: buf.len(),
        });
    }

    let input = u64::from_be_bytes(buf);
    let res = config.get_number(input);

    debug!("Got number {input}, sending {res}");
    socket.send_to(&res.to_be_bytes(), address)?;

    Ok(())
}
