mod error;
mod message;

use server_utils::cli;
use std::error::Error;
use std::net::{Ipv4Addr, UdpSocket};
use tracing::{info, warn};

fn run() -> Result<(), Box<dyn Error>> {
    let config = cli::env_config()?;

    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 9999))?;
    info!("Bound to {}", socket.local_addr().unwrap());

    loop {
        let Err(e) = message::handle_datagram(&config, &socket) else {
            continue;
        };

        warn!("Error handling datagram: {e}");
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    cli::exec(run);
}
