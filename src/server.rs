use crate::connection::Connection;
use crate::event::Event;
use crate::event::EventScheduler;
use crate::map::Map;
use crate::pending::PendingClient;
use crate::player::{Player, PlayerState};
use crate::protocol::PendingResponse::{LogAs, Shared};
use crate::protocol::{
    AIAction, AIResponse, ClientSender, EventType, GameEvent, HasId, Id, PendingAction,
    ServerResponse, SharedAction, SharedResponse, TeamType,
};
use crate::resources::Resource;
use crate::sound::get_sound_direction;
use crate::team::Team;
use crate::vec2::{HasPosition, Size};
use log::{debug, info, warn};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
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

pub struct ThreadChannel<T> {
    pub(crate) tx: mpsc::Sender<T>,
    pub(crate) rx: mpsc::Receiver<T>,
}

pub struct Server {
    global_channel: ThreadChannel<EventType>,
    tick_interval: time::Interval,
    socket: TcpListener,
    freq: u64,
    map: Map,
    teams: HashMap<Id, Team>,
    pending_clients: HashMap<Id, PendingClient>,
    clients: HashMap<Id, Player>,
    event_scheduler: EventScheduler<Event>,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("socket error: {0}")]
    FailedToBind(#[from] std::io::Error),
}

const SATIETY_LOSS_PER_TICK: u64 = 1;

impl Server {
    pub async fn from_config(config: ServerConfig) -> Result<Server, ServerError> {
        let addr = format!("{}:{}", config.addr, config.port);
        let socket = TcpListener::bind(addr).await?;
        let (tx, rx) = mpsc::channel::<EventType>(32);
        let tick_interval = time::interval(time::Duration::from_nanos(
            (1_000_000_000f64 / config.freq as f64) as u64,
        ));

        let mut teams: HashMap<Id, Team> = HashMap::new();

        for (team_id, team) in config.teams.into_iter().enumerate() {
            teams.insert(team_id as Id, Team::new(team_id as Id, team));
        }

        let mut map = Map::new(Size::new(config.width as u64, config.height as u64));

        for (team_id, ..) in &teams {
            map.spawn_eggs(*team_id, config.clients_nb);
        }

        Ok(Server {
            global_channel: ThreadChannel { tx, rx },
            tick_interval,
            socket,
            freq: config.freq as u64,
            map,
            teams,
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            event_scheduler: EventScheduler::new(),
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
        let size_x = self.map.size().x();
        let size_y = self.map.size().y();

        let total: u64 = size_x * size_y;
        let resources: [(Resource, u64); 7] = [
            (Resource::Food, (0.5 * total as f64) as u64),
            (Resource::Linemate, (0.3 * total as f64) as u64),
            (Resource::Deraumere, (0.15 * total as f64) as u64),
            (Resource::Sibur, (0.1 * total as f64) as u64),
            (Resource::Mendiane, (0.1 * total as f64) as u64),
            (Resource::Phiras, (0.08 * total as f64) as u64),
            (Resource::Thystame, (0.05 * total as f64) as u64),
        ];

        for res in Resource::iter() {
            if self.map.resources()[res] >= resources[res as usize].1 {
                continue;
            }
            let nb_missing = resources[res as usize].1 - self.map.resources()[res];
            (0..nb_missing).for_each(|_| {
                let x = rand::rng().random_range(0..size_x);
                let y = rand::rng().random_range(0..size_y);
                self.map.add_resource(res, 1, (x, y).into());
            });
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
                    self.update(instant).await;
                },

                Some(res) = self.global_channel.rx.recv() => {
                    self.process_events(res).await;
                },
            }
        }
    }

    fn accept_client(&mut self, socket: TcpStream, addr: SocketAddr) {
        static CLIENT_ID: AtomicU64 = AtomicU64::new(0);
        let client_id: Id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        info!(
            "Accepted connection from {:?} with id {}",
            socket.peer_addr().unwrap(),
            client_id
        );
        let server_tx = self.global_channel.tx.clone();
        let (client_tx, client_rx) = mpsc::channel::<ServerResponse>(32);
        self.pending_clients.insert(
            client_id,
            PendingClient {
                client_id,
                client_tx,
            },
        );
        tokio::spawn(async move {
            let (mut client, read_half) = Connection::new(client_id, socket, server_tx).await;
            client.handle(client_rx, read_half).await
        });
    }

    async fn update(&mut self, _instant: time::Instant) {
        //info!("Updating current tick {:?}", self.event_scheduler.current_tick());
        //info!("Updating server {}", self.clients.len());
        //print!("\x1B[2J\x1B[1;1H"); // Effacer l'écran et replacer le curseur en haut à gauche
        //println!("{}", self.map);
        //println!("{:?}", self.clients);
        self.spawn_resources();
        let expired_events = self.event_scheduler.tick();
        for timed_event in expired_events {
            // do or ignore event if dead
            match timed_event.data {
                Event::Broadcast(str) => {
                    let Some(emitter) = self.clients.get(&timed_event.player_id) else {
                        continue;
                    };
                    let str = Arc::new(str);
                    for receiver in self
                        .clients
                        .values()
                        .filter(|receiver| receiver.id() != emitter.id())
                    {
                        let dir =
                            get_sound_direction(emitter.into(), receiver.into(), self.map.size());
                        let _ = receiver.send_to_client(ServerResponse::AI(AIResponse::Broadcast(
                            dir,
                            str.clone(),
                        )));
                    }
                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                }
                Event::Forward => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    emitter
                        .move_forward(&self.map.size())
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                }
                Event::Right => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    emitter.direction_mut().rotate_right();
                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                }
                Event::Left => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    emitter.direction_mut().rotate_left();
                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                }
                Event::Look => {
                    unreachable!()
                }
                Event::Inventory => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    emitter.send_to_client(ServerResponse::AI(AIResponse::Inventory(
                        emitter.inventory(),
                    )));
                }
                Event::ConnectNbr => {
                    unreachable!()
                }
                Event::Fork => {
                    unreachable!()
                }
                Event::Eject => {
                    unreachable!()
                }
                Event::Take(resource) => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    match self.map.del_resource(resource, 1, emitter.position()) {
                        None => {
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ko,
                            )));
                        }
                        Some(_) => {
                            emitter
                                .add_resource(resource, 1)
                                .send_to_client(ServerResponse::AI(AIResponse::Shared(
                                    SharedResponse::Ok,
                                )));
                        }
                    };
                }
                Event::Set(resource) => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    if emitter.state() == PlayerState::Incantating {
                        continue;
                    }
                    let res = emitter.del_resource(resource, 1);
                    match res {
                        None => {
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ko,
                            )));
                        }
                        Some(resource) => {
                            self.map.add_resource(resource, 1, emitter.position());
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ok,
                            )));
                        }
                    }
                }
                Event::Incantation => {
                    unreachable!()
                }
                Event::Ko => {
                    if let Some(client) = self.clients.get_mut(&timed_event.player_id) {
                        client.send_to_client(ServerResponse::AI(AIResponse::Shared(
                            SharedResponse::Ko,
                        )));
                    } else {
                        continue;
                    }
                }
            }
        }
        self.reduce_satiety();
    }

    pub fn reduce_satiety(&mut self) {
        for (id, client) in self.clients.iter_mut() {
            if client.reduce_satiety(SATIETY_LOSS_PER_TICK) == 0 {
                client.send_to_client(ServerResponse::AI(AIResponse::Dead));
                info!("Client {} is dead", client.id());
            }
        }
    }

    async fn process_events(&mut self, event: EventType) {
        debug!("Event {:?}", event);
        match event {
            EventType::AI(GameEvent { id, action }) => {
                self.handle_ai_events((id, action)).await;
            }
            EventType::GUI(GameEvent { id, action }) => {
                unreachable!()
            }
            EventType::Pending(GameEvent { id, action }) => {
                self.handle_pending_events((id, action)).await;
            }
        }
    }

    async fn handle_pending_events(&mut self, (id, action): (Id, PendingAction)) {
        let Some(client) = self.pending_clients.get_mut(&id) else {
            warn!(
                "This client is not pending anymore : {}, cancelled event {:?}",
                id, action
            );
            return;
        };

        fn send_ko(client: &mut impl ClientSender) {
            client.send_to_client(ServerResponse::Pending(Shared(SharedResponse::Ko)));
        }

        match action {
            PendingAction::Shared(SharedAction::Disconnected) => {
                self.pending_clients.remove_entry(&id);
                info!("Pending client: {} disconnected", id);
            }
            PendingAction::Shared(SharedAction::InvalidAction) => unreachable!(),
            PendingAction::Shared(SharedAction::ReachedTakeLimit) => {
                warn!("Pending client: {} sent too much data", id);
                send_ko(client);
            }
            PendingAction::Shared(SharedAction::InvalidEncoding) => {
                warn!("Pending client: {} uses invalid encoding", id);
                send_ko(client);
            }
            PendingAction::Login(team_name) => {
                let Some(team) = self.teams.values().find(|team| team.name() == team_name) else {
                    send_ko(client);
                    return;
                };

                if self.map.nb_eggs_by_team(team.id()) == 0 {
                    warn!(
                        "Client {} can't login: team '{}' has no eggs",
                        id, team_name
                    );
                    send_ko(client);
                    return;
                }

                let egg = self.map.drop_egg(team.id()).unwrap();
                let pending_client = self.pending_clients.remove(&id).unwrap();

                let player_builder = Player::builder()
                    .team(team.id())
                    .pending_client(pending_client)
                    .position(egg.position());

                let player = player_builder.build().unwrap();
                player.send_to_client(ServerResponse::Pending(LogAs(TeamType::IA(
                    self.map.nb_eggs_by_team(team.id()),
                    player.position(),
                ))));

                self.clients.insert(player.id(), player);
            }
        }
    }

    async fn handle_ai_events(&mut self, (id, action): (Id, AIAction)) {
        match action {
            AIAction::Shared(shared) => match shared {
                SharedAction::Disconnected => {
                    self.clients.remove(&id);
                    self.event_scheduler.del_player(id);
                }
                SharedAction::InvalidAction
                | SharedAction::ReachedTakeLimit
                | SharedAction::InvalidEncoding => {
                    let event: Event = Event::Ko;
                    self.event_scheduler.schedule(event, 0, id);
                }
            },
            AIAction::Action(action) => match action {
                event @ (Event::Broadcast(_)
                | Event::Forward
                | Event::Right
                | Event::Left
                | Event::Look
                | Event::Take(_)
                | Event::Set(_)
                | Event::Eject) => {
                    self.event_scheduler.schedule(event, 7, id);
                }
                event @ Event::Inventory => {
                    self.event_scheduler.schedule(event, 1, id);
                }
                event @ Event::ConnectNbr => {
                    self.event_scheduler.schedule(event, 0, id);
                }
                event @ Event::Fork => {
                    self.event_scheduler.schedule(event, 42, id);
                }
                event @ Event::Incantation => {
                    todo!()
                }
                _ => {
                    unreachable!()
                }
            },
        }
    }
}
