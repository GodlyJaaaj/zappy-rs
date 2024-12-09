use crate::client::Client;
use tokio::net::TcpListener;
use core::unimplemented;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

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
    tcp_listener: TcpListener,
    clients: HashMap<usize, Client>, //replace by hashmap
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
	unimplemented!();
    }

    fn accept_client(tcp_listener: &TcpListener) -> Result<Client, Box<dyn Error>> {
	unimplemented!();
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
	unimplemented!();
        // let mut clients = SERVER.0 + 1;
        // loop {
        //     self.poll.poll(&mut self.events, None)?;
        //     for event in self.events.iter() {
        //         match event.token() {
        //             SERVER => {
        //                 let Ok(mut new_client)= Server::accept_client(&self.tcp_listener) else {
        //                     eprintln!("Failed to accept client");
        //                     continue;
        //                 };
        //                 self.poll.registry().register(&mut new_client.socket,
        //                                               Token(clients),
        //                                               mio::Interest::READABLE)?;
        //                 self.clients.insert(clients, new_client);
        //                 clients += 1;
        //             },
        //             _ => {
        //                 if event.is_readable() {
        //                     let mut buf = [0; 1024];
        //                     let client = self.clients.get_mut(&event.token().0).unwrap();
        //                     match client.socket.read(&mut buf) {
        //                         Ok(0) => {
        //                             println!("Client disconnected");
        //                             self.poll.registry().deregister(&mut client.socket)?;
        //                             self.clients.remove(&event.token().0);
        //                         },
        //                         Ok(n) => {
        //                             println!("Read {} bytes", n);
        //                         },
        //                         Err(e) => {
        //                             eprintln!("Failed to read from client {}:{}", e, e.kind());
        //                         }
        //                     }
        //                 }
        //             },
        //         }
        //     }
        //     self.events.clear();
        // }
    }
}
