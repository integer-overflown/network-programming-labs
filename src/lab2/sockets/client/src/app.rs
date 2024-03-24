use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream, ToSocketAddrs, UdpSocket};
use std::num::{NonZeroUsize, ParseIntError};
use std::os::fd::{AsFd, AsRawFd};

use clap::{arg, Parser};
use polling::{Event, Events, Poller};
use tracing::warn;

const SOCKET_KEY: usize = 0;
const STDIN_KEY: usize = 1;

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'u', long = "udp", default_value_t = false)]
    udp: bool,

    host: String,
    port: u16,
}

enum ClientSocket {
    Udp(UdpSocket),
    Tcp(TcpStream),
}

trait Pollable {
    fn add_to_poller(&self, poller: &Poller, key: usize) -> io::Result<()>;
    fn modify_interest(&self, poller: &Poller, key: usize) -> io::Result<()>;
}

impl ClientSocket {
    fn new(server_address: impl ToSocketAddrs, udp: bool) -> io::Result<ClientSocket> {
        Ok(if udp {
            let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 8888))?;
            socket.connect(server_address)?;
            socket.set_nonblocking(true)?;

            ClientSocket::Udp(socket)
        } else {
            let socket = TcpStream::connect(server_address)?;
            socket.set_nonblocking(true)?;

            ClientSocket::Tcp(socket)
        })
    }

    fn submit_input(&mut self, input: &[u8]) -> io::Result<()> {
        match self {
            ClientSocket::Tcp(socket) => {
                socket.write_all(input)?;
                socket.flush()?;
            }
            ClientSocket::Udp(socket) => {
                socket.send(input)?;
            }
        }

        Ok(())
    }
}

impl Pollable for ClientSocket {
    fn add_to_poller(&self, poller: &Poller, key: usize) -> io::Result<()> {
        let ev = Event::readable(key);

        unsafe {
            match self {
                Self::Udp(socket) => poller.add(socket, ev),
                Self::Tcp(socket) => poller.add(socket, ev),
            }
        }
    }

    fn modify_interest(&self, poller: &Poller, key: usize) -> io::Result<()> {
        let ev = Event::readable(key);

        match self {
            Self::Udp(socket) => poller.modify(socket, ev),
            Self::Tcp(socket) => poller.modify(socket, ev),
        }
    }
}

impl<T: AsRawFd + AsFd> Pollable for T {
    fn add_to_poller(&self, poller: &Poller, key: usize) -> io::Result<()> {
        unsafe { poller.add(self, Event::readable(key)) }
    }

    fn modify_interest(&self, poller: &Poller, key: usize) -> io::Result<()> {
        poller.modify(self, Event::readable(key))
    }
}

pub struct App {
    client: ClientSocket,
    stdin: io::Stdin,
    poller: Poller,
}

#[derive(Debug)]
pub enum SourceError {
    IoError(io::Error),
    InvalidNumber(ParseIntError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceError::IoError(_) => {
                write!(f, "I/O error occurred")
            }
            SourceError::InvalidNumber(_) => {
                write!(f, "Invalid number")
            }
        }
    }
}

impl Error for SourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SourceError::IoError(e) => Some(e),
            SourceError::InvalidNumber(e) => Some(e),
        }
    }
}

impl From<io::Error> for SourceError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for SourceError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value)
    }
}

impl App {
    pub fn new(args: Args) -> io::Result<App> {
        let client = ClientSocket::new((args.host, args.port), args.udp)?;
        let poller = Poller::new()?;
        let stdin = io::stdin();

        client.add_to_poller(&poller, SOCKET_KEY)?;
        stdin.add_to_poller(&poller, STDIN_KEY)?;

        Ok(App {
            client,
            stdin,
            poller,
        })
    }

    fn read_source(&mut self, key: usize) -> Result<(), SourceError> {
        match key {
            SOCKET_KEY => {
                let mut buf = [0; server_utils::message::MESSAGE_SIZE];

                match &mut self.client {
                    ClientSocket::Tcp(socket) => {
                        socket.read_exact(&mut buf)?;
                    }
                    ClientSocket::Udp(socket) => {
                        let received = socket.recv(&mut buf)?;

                        if received < buf.len() {
                            return Err(SourceError::IoError(io::Error::from(UnexpectedEof)));
                        }
                    }
                };

                let num = u64::from_be_bytes(buf);
                println!("Have reply from remote: {}", num);
            }
            STDIN_KEY => {
                self.stdin.modify_interest(&self.poller, STDIN_KEY)?;

                let mut input = String::new();
                self.stdin.read_to_string(&mut input)?;

                let input: u64 = input.trim().parse()?;
                println!("\n[I] Sending {input} to the remote");

                self.client.submit_input(&input.to_be_bytes())?;
                self.client.modify_interest(&self.poller, SOCKET_KEY)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn start_loop(&mut self) -> io::Result<App> {
        let mut events = Events::with_capacity(NonZeroUsize::new(2).unwrap());

        loop {
            print!("> ");
            io::stdout().flush()?;

            events.clear();
            self.poller.wait(&mut events, None)?;

            for ev in events.iter() {
                if let Err(e) = self.read_source(ev.key) {
                    warn!("Failed to read data: {e}, caused by {:?}", e.source());
                }
            }
        }
    }
}
