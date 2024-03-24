use server_utils::{cli, message};
use std::error::Error;

use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use task::Config;
use tracing::{debug, info, warn};

mod error;

fn handle_connection(
    config: &Config,
    mut connection: TcpStream,
) -> Result<(), error::ConnectionError> {
    debug!(
        "Handling new connection, peer addr: {:?}",
        connection.peer_addr()
    );

    let input = message::read_number(&mut connection)?;
    let res = config.get_number(input);

    debug!("Got {input}, sending {res}");

    connection.write_all(&res.to_be_bytes())?;

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = cli::env_config()?;

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 7878)).unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(value) => value,
            Err(e) => {
                warn!("Connection failed: {e}");
                continue;
            }
        };

        let Err(err) = handle_connection(&config, stream) else {
            continue;
        };

        warn!("Could not handle incoming connection: {err}");

        if let Some(source) = err.source() {
            warn!("Caused by: {source}");
        }
    }

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();
    cli::exec(run);
}
