mod error;
mod message;

use std::net::{Ipv4Addr, UdpSocket};
use tracing::{info, warn};

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 9999))?;
    info!("Bound to {}", socket.local_addr().unwrap());

    loop {
        let Err(e) = message::receive_message(&socket) else {
            continue;
        };

        warn!("Error handling datagram: {e}");
    }
}
