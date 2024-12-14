use crate::connection::Connection;
use crate::protocol::ClientAction;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::{select, time};
use crate::map::Map;
use crate::team::Team;
use crate::vec2::Size;

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

struct ThreadChannel {
    tx: mpsc::Sender<ClientAction>,
    rx: mpsc::Receiver<ClientAction>,
}

pub struct Server {
    thread_channel: ThreadChannel,
    tick_interval: time::Interval,
    socket: TcpListener,
    freq: u64,
    map: Map,
    max_clients: u64,
    teams: Vec<Team>,
}

#[derive(Debug)]
pub enum ServerError {
    FailedToBind,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ServerError {}

impl Server {
    pub async fn from_config(config: ServerConfig) -> Result<Server, ServerError> {
        let addr = format!("{}:{}", config.addr, config.port);
        let socket = TcpListener::bind(addr)
            .await
            .map_err(|_| ServerError::FailedToBind)?;
        let (tx, rx) = mpsc::channel::<ClientAction>(32);
        let thread_channel = ThreadChannel { tx, rx };
        let tick_interval = time::interval(time::Duration::from_nanos(
            (1_000_000_000f64 / config.freq as f64) as u64,
        ));
        let mut teams = vec![];
        for team in config.teams {
            teams.push(Team::new(teams.len(), team, config.clients_nb));
        }

        Ok(Server {
            thread_channel,
            tick_interval,
            socket,
            freq: config.freq as u64,
            map: Map::new(Size::new(config.width as u64, config.height as u64)),
            max_clients: config.clients_nb,
            teams
        })
    }

    fn set_tick_interval(&mut self, freq: u16) {
        self.freq = freq as u64;
        let freq = (1_000_000_000f64 / freq as f64) as u64;
        self.tick_interval = time::interval(time::Duration::from_nanos(freq));
    }

    fn freq(&self) -> u64 {
        self.freq
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            select! {
                biased;

                Ok((socket, addr)) = self.socket.accept() => {
                    self.accept_client(socket, addr);
                },

                instant = self.tick_interval.tick() => {
                    self.update(instant)
                },

                Some(res) = self.thread_channel.rx.recv() => {
                    self.process_events(res).await;
                },
            }
        }
    }

    fn accept_client(&mut self, socket: TcpStream, addr: SocketAddr) {
        println!("Accepted connection from {:?}", socket);
        let ctx = self.thread_channel.tx.clone();
        tokio::spawn(async move {
            let mut client = Connection::new(socket);
            client.handle(ctx).await
        });
    }

    fn update(&mut self, _instant: time::Instant) {
        println!("Server tick!");
    }

    async fn process_events(&mut self, action: ClientAction) {
        println!("Processing action {:?}", action);
    }
}
