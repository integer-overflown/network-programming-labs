use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use tracing::{debug, info, warn};

mod error;

fn handle_connection(mut connection: TcpStream) -> Result<(), error::ConnectionError> {
    let mut message = Vec::new();
    connection.read_to_end(&mut message)?;

    let message = String::from_utf8(message)?;
    debug!("Received message: {message:?}");

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        info!("Handling new connection");

        let Err(err) = stream.map(handle_connection) else {
            continue;
        };

        warn!("Could not handle incoming connection: {err}");

        if let Some(source) = err.source() {
            warn!("Caused by {source}");
        }
    }
}
