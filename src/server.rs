use crate::client::Client;
use crate::protocol::ClientAction;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
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
    config: ServerConfig,
    ticks: u64,
    clients: HashMap<usize, Client>, //replace by hashmap
    //freq: u16,
    //teams
}

#[derive(Debug)]
pub enum ServerError {
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
	todo!()
    }
}

impl Error for ServerError {}

impl Server {
    pub fn from_config(config: ServerConfig) -> Server {
	Server {
	    config,
	    ticks: 0,
	    clients: HashMap::new(),
	}
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
	let addr = format!("{}:{}", self.config.addr, self.config.port);
	let listener = TcpListener::bind(addr).await?;
	let (tx, rx) = mpsc::channel(32);

	tokio::spawn(async move {
	    Self::handle_connections(listener, tx).await
	});
	self.process_events(rx).await;
	Ok(())
    }

    async fn handle_connections(listener: TcpListener, tx: mpsc::Sender<ClientAction>) {
	loop {
	    tx.send(ClientAction::Forward).await.unwrap();
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
	    let ctx = tx.clone();
	    tokio::spawn(async move {
		Self::process_connection(socket, ctx).await;
	    });
	}
    }

    async fn process_connection(socket: TcpStream, tx: mpsc::Sender<ClientAction>) {
	let client = Client::new(socket);
	loop{}
    }

    async fn process_events(&mut self, mut rx: mpsc::Receiver<ClientAction>) {
	loop {
	    let action = rx.recv().await;
	    println!("Processing action {:?}", action);
	}
    }
}
