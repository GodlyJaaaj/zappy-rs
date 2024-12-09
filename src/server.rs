use crate::client::Client;
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tokio::io;

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
    ticks: u64,
    tcp_listener: TcpListener,
    clients: HashMap<usize, Client>, //replace by hashmap
    //freq: u16,
    //teams
}

#[derive(Debug)]
pub enum ServerError {
    FailedToParseAddr,
    FailedToBind(std::io::Error),
    FailedToMakeReadable,
    FailedToMakeUnreadable,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::FailedToParseAddr => write!(f, "Failed to parse address"),
            ServerError::FailedToBind(e) => write!(f, "Failed to bind address: {}", e),
            ServerError::FailedToMakeReadable => write!(f, "Failed to make server readable"),
            ServerError::FailedToMakeUnreadable => write!(f, "Failed to make server unreadable"),
        }
    }
}

impl Error for ServerError {}

impl Server {
    pub async fn from_config(config: ServerConfig) -> io::Result<Server> {
	let server = Server {
	    ticks: 0,
	    tcp_listener: TcpListener::bind(config.addr).await?,
	    clients: HashMap::new(),
	};

	Ok(server)
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
	loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = self.tcp_listener.accept().await.unwrap();
            Self::accept_client(socket).await.ok();
	}
    }

    async fn accept_client(socket: TcpStream) -> Result<Client, Box<dyn Error>> {
	println!("New client");
	Ok(Client::new(socket))
    }
}
