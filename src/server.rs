use crate::client::Client;
use mio::{net::TcpListener, Events, Poll, Token};
use std::error::Error;
use std::fmt::{Display, Formatter};

const SERVER: Token = Token(0);

pub struct ServerConfig {
    addr: String,
    port: u16,
    width: u8,
    height: u8,
    teams: Vec<String>,
    clients_nb: u64,
    freq: u16,
}

impl ServerConfig {
    pub fn new(
        addr: String,
        port: u16,
        width: u8,
        height: u8,
        teams: Vec<String>,
        clients_nb: u64,
        freq: u16,
    ) -> Self {
        ServerConfig {
            addr,
            port,
            width,
            height,
            teams,
            clients_nb,
            freq,
        }
    }
}

pub struct Server {
    ticks: u128,
    poll: Poll,
    events: Events,
    tcp_listener: TcpListener,
    clients: Vec<Client>,
    //freq: u16,
    //teams
}

#[derive(Debug)]
pub enum ServerError {
    PollError(std::io::Error),
    FailedToParseAddr,
    FailedToBind,
    FailedToMakeReadable,
    FailedToMakeUnreadable,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::PollError(e) => write!(f, "Failed to create poll: {}", e),
            ServerError::FailedToParseAddr => write!(f, "Failed to parse address"),
            ServerError::FailedToBind => write!(f, "Failed to bind address"),
            ServerError::FailedToMakeReadable => write!(f, "Failed to make server readable"),
            ServerError::FailedToMakeUnreadable => write!(f, "Failed to make server unreadable"),
        }
    }
}

impl Error for ServerError {}

impl Server {
    pub fn from_config(config: ServerConfig) -> Result<Self, ServerError> {
        let server = Server {
            ticks: 0,
            poll: Poll::new().map_err(ServerError::PollError)?,
            events: Events::with_capacity(2048),
            tcp_listener: TcpListener::bind(
                format!("{}:{}", config.addr, config.port)
                    .parse()
                    .map_err(|_| ServerError::FailedToParseAddr)?,
            )
            .map_err(|_| ServerError::FailedToBind)?,
            clients: Vec::new(),
        };

        Ok(server)
    }

    /// Try to make the server readable
    /// This will allow the server to accept new connections
    pub fn try_make_readable(&mut self) -> Result<(), ServerError> {
        self.poll
            .registry()
            .register(&mut self.tcp_listener, SERVER, mio::Interest::READABLE)
            .map_err(|_| ServerError::FailedToMakeReadable)
    }

    /// Try to make the server unreadable
    /// This will prevent the server from accepting new connections
    pub fn try_make_unreadable(&mut self) -> Result<(), ServerError> {
        self.poll
            .registry()
            .deregister(&mut self.tcp_listener)
            .map_err(|_| ServerError::FailedToMakeUnreadable)
    }
    
    fn accept_client(&mut self) {
        match self.tcp_listener.accept() {
            Ok((socket, _)) => {
                self.clients.push(Client::new(socket));
            }
            Err(e) => {
                eprintln!("Failed to accept client {}", e);
            }
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.poll.poll(&mut self.events, None)?;
            for event in self.events.iter() {
                if event.token() != SERVER {
                    match self.tcp_listener.accept() {
                        Ok((socket, _)) => {
                            self.clients.push(Client::new(socket));
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}
