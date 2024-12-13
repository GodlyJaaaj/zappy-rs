use crate::connection::Connection;
use crate::protocol::ClientAction;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::{select, time};

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
}

#[derive(Debug)]
pub enum ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ServerError {}

impl Server {
    pub fn from_config(config: ServerConfig) -> Server {
        Server { config }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.config.addr, self.config.port);
        let listener = TcpListener::bind(addr).await?;
        let (tx, mut rx) = mpsc::channel::<ClientAction>(32);

        let mut interval = time::interval(time::Duration::from_secs(1));
        loop {
            select! {
                biased;
                Ok((socket, _)) = listener.accept() => {
                    println!("Accepted connection from {:?}", socket);
                    let ctx = tx.clone();
                    tokio::spawn(async move {
                        let mut client = Connection::new(socket);
                        client.handle(ctx).await
                    });
                },
                _ = interval.tick() => {
                        println!("Server tick!");
                    },

                Some(res) = rx.recv() => {
                    self.process_events(res).await;
                },
            }
        }
    }

    async fn process_events(&mut self, action: ClientAction) {
        println!("Processing action {:?}", action);
    }
}
