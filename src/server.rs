use crate::connection::Connection;
use crate::map::Map;
use crate::pending::PendingClient;
use crate::player::Player;
use crate::protocol::{Action, ClientAction, ClientType, Ko};
use crate::sound::get_sound_direction;
use crate::team::Team;
use crate::vec2::Size;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::{select, time};
use crate::resources::{Resource, Resources};

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

pub struct ThreadChannel {
    pub(crate) tx: mpsc::Sender<ClientAction>,
    pub(crate) rx: mpsc::Receiver<ClientAction>,
}

pub struct Server {
    thread_channel: ThreadChannel,
    tick_interval: time::Interval,
    socket: TcpListener,
    freq: u64,
    map: Map,
    max_clients: u64,
    teams: HashMap<u64, Team>,
    pending_clients: HashMap<u64, PendingClient>,
    clients: HashMap<u64, Player>,
    resource: Resources,
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
        let tick_interval = time::interval(time::Duration::from_nanos(
            (1_000_000_000f64 / config.freq as f64) as u64,
        ));

        let mut teams: HashMap<u64, Team> = HashMap::new();

        for (team_id, team) in config.teams.into_iter().enumerate() {
            teams.insert(team_id as u64, Team::new(team_id as u64, team));
        }

        Ok(Server {
            thread_channel: ThreadChannel { tx, rx },
            tick_interval,
            socket,
            freq: config.freq as u64,
            map: Map::new(Size::new(config.width as u64, config.height as u64)),
            max_clients: config.clients_nb,
            teams,
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            resource: Resources::default(),
        })
    }

    // resource density
    // food 0.5
    // linemate 0.3
    // deraumere 0.15
    // sibur 0.1
    // mendiane 0.1
    // phiras 0.08
    // thystame 0.05
    fn spawn_resources(&mut self) {
        for i in 0..=8 {
            
        }
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
        static CLIENT_ID: AtomicU64 = AtomicU64::new(0);
        let client_id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        println!(
            "Accepted connection from {:?} with id {}",
            socket, client_id
        );
        let _ = socket.try_write(b"WELCOME\n");
        let server_tx = self.thread_channel.tx.clone();
        let (client_tx, client_rx) = mpsc::channel::<ClientAction>(32);
        self.pending_clients.insert(
            client_id,
            PendingClient {
                client_id,
                client_tx,
            },
        );
        tokio::spawn(async move {
            let mut client = Connection::new(client_id, socket);
            client.handle(server_tx, client_rx).await
        });
    }

    fn update(&mut self, _instant: time::Instant) {
        //println!("Server tick!");
    }

    async fn add_player(&mut self, player: Player) {
        player
            .send(ClientAction {
                client_id: player.id(),
                action: Action::LoggedIn(
                    ClientType::AI,
                    0, // todo! calculate number of eggs
                    self.map.size(),
                ),
            })
            .await;
        self.clients.insert(player.id(), player);
    }

    async fn process_events(&mut self, action: ClientAction) {
        println!("Processing action {:?}", action.action);
        match action.action {
            Action::LoggedIn(_, _, _) => unreachable!("Thread should not send LoggedIn action"),
            Action::Ok => {
                unreachable!("Client should not send Ok action")
            }
            Action::Ko => {
                unreachable!("Client should not send Ko action")
            }
            Action::Broadcast(dir, message) => {
                // get the player that sent the broadcast
                let Some(emitter) = self.clients.get(&action.client_id) else {
                    unreachable!("Client should be in clients");
                };
                for receiver in self.clients.values() {
                    let dir = get_sound_direction(emitter.into(), receiver.into(), self.map.size());
                    receiver
                        .send(ClientAction {
                            client_id: emitter.id(),
                            action: Action::Broadcast(dir, message.clone()),
                        })
                        .await;
                }
            }
            Action::Forward => {
                let player = self.clients.get_mut(&action.client_id);
                let Some(player) = player else {
                    return;
                };
                player.move_forward(&self.map.size());
                player
                    .send(ClientAction {
                        client_id: player.id(),
                        action: Action::Ok,
                    })
                    .await;
            }
            Action::Right => {
                let player = self.clients.get_mut(&action.client_id);
                let Some(player) = player else {
                    return;
                };
                player.direction_mut().rotate_right();
                player
                    .send(ClientAction {
                        client_id: player.id(),
                        action: Action::Ok,
                    })
                    .await;
            }
            Action::Left => {
                let player = self.clients.get_mut(&action.client_id);
                let Some(player) = player else {
                    return;
                };
                player.direction_mut().rotate_left();
                player
                    .send(ClientAction {
                        client_id: player.id(),
                        action: Action::Ok,
                    })
                    .await;
            }
            Action::Look => {
                todo!("Implement look")
            }
            Action::Inventory(_) => {
                let player = self.clients.get(&action.client_id);
                let Some(player) = player else {
                    return;
                };
                player
                    .send(ClientAction {
                        client_id: player.id(),
                        action: Action::Inventory(player.inventory()),
                    })
                    .await;
            }
            Action::ConnectNbr => {
                todo!("Implement connect_nbr")
            }
            Action::Fork => {
                todo!("Implement fork")
            }
            Action::Eject => {
                todo!("Implement eject")
            }
            Action::Take(_) => {
                todo!("Implement take")
            }
            Action::Set(_) => {
                todo!("Implement set")
            }
            Action::Incantation => {
                todo!("Implement incantation")
            }
            Action::Disconnect => {
                self.pending_clients.remove(&action.client_id); // ensure client is removed
                self.clients.remove(&action.client_id); // ensure client is removed
                println!("{:?}", self.pending_clients);
                println!("{:?}", self.clients);
            }
            Action::Login(team_name) => {
                //todo! gui team
                let pending_client = self.pending_clients.remove(&action.client_id);
                let Some(mut pending_client) = pending_client else {
                    //client disconnected in between
                    return;
                };
                let team = self
                    .teams
                    .values_mut()
                    .find(|team| team.name() == team_name);
                let Some(team) = team else {
                    pending_client.ko().await;
                    self.pending_clients
                        .insert(action.client_id, pending_client);
                    return;
                };
                let player = Player::new(team.id(), pending_client);
                self.add_player(player).await;
            }
        }
    }
}
