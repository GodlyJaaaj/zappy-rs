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
use crate::resources::{Resource, Resources};
use crate::sound::get_sound_direction;
use crate::team::Team;
use crate::vec2::Size;
use log::{debug, info, warn};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
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
    max_clients: u64,
    teams: HashMap<Id, Team>,
    pending_clients: HashMap<Id, PendingClient>,
    clients: HashMap<Id, Player>,
    resources: Resources,
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

        Ok(Server {
            global_channel: ThreadChannel { tx, rx },
            tick_interval,
            socket,
            freq: config.freq as u64,
            map: Map::new(Size::new(config.width as u64, config.height as u64)),
            max_clients: config.clients_nb,
            teams,
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            resources: Resources::default(),
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
            if self.resources[res] >= resources[res as usize].1 {
                continue;
            }
            let nb_missing = resources[res as usize].1 - self.resources[res];
            (0..nb_missing).for_each(|_| {
                let x = rand::rng().random_range(0..size_x);
                let y = rand::rng().random_range(0..size_y);
                self.map[(x, y)].add_resource(res, 1);
            });
            self.resources[res] += nb_missing;
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
                    info!("Sent AI event");
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
                Event::Look => {}
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
                Event::ConnectNbr => {}
                Event::Fork => {}
                Event::Eject => {}
                Event::Take(_) => {}
                Event::Set(_) => {}
                Event::Incantation => {}
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
                info!("Pending client {} logged in with team {}", id, team_name);

                let Some(team) = self.teams.values().find(|team| team.name() == team_name) else {
                    send_ko(client);
                    return;
                };

                let pending_client = self.pending_clients.remove(&id).unwrap();
                let player = Player::new(team.id(), pending_client);
                warn!("hardcoded info at login");
                player.send_to_client(ServerResponse::Pending(LogAs(TeamType::IA(
                    team_name,
                    0,
                    (0, 0).into(),
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

//match action.action {
//             Action::LoggedIn(_, _, _) => unreachable!("Thread should not send LoggedIn action"),
//             Action::Ok => {
//                 unreachable!("Client should not send Ok action")
//             }
//             Action::Ko => {
//                 unreachable!("Client should not send Ko action")
//             }
//             Action::Broadcast(dir, message) => {
//                 // get the player that sent the broadcast
//                 let Some(emitter) = self.clients.get(&action.client_id) else {
//                     unreachable!("Client should be in clients");
//                 };
//                 for receiver in self.clients.values() {
//                     let dir = get_sound_direction(emitter.into(), receiver.into(), self.map.size());
//                     receiver
//                         .send(ClientAction {
//                             client_id: emitter.id(),
//                             action: Action::Broadcast(dir, message.clone()),
//                         })
//                         .await;
//                 }
//             }
//             Action::Forward => {
//                 let player = self.clients.get_mut(&action.client_id);
//                 let Some(player) = player else {
//                     return;
//                 };
//                 player.move_forward(&self.map.size());
//                 player
//                     .send(ClientAction {
//                         client_id: player.id(),
//                         action: Action::Ok,
//                     })
//                     .await;
//             }
//             Action::Right => {
//                 let player = self.clients.get_mut(&action.client_id);
//                 let Some(player) = player else {
//                     return;
//                 };
//                 player.direction_mut().rotate_right();
//                 player
//                     .send(ClientAction {
//                         client_id: player.id(),
//                         action: Action::Ok,
//                     })
//                     .await;
//             }
//             Action::Left => {
//                 let player = self.clients.get_mut(&action.client_id);
//                 let Some(player) = player else {
//                     return;
//                 };
//                 player.direction_mut().rotate_left();
//                 player
//                     .send(ClientAction {
//                         client_id: player.id(),
//                         action: Action::Ok,
//                     })
//                     .await;
//             }
//             Action::Look => {
//                 todo!("Implement look")
//             }
//             Action::Inventory(_) => {
//                 let player = self.clients.get(&action.client_id);
//                 let Some(player) = player else {
//                     return;
//                 };
//                 player
//                     .send(ClientAction {
//                         client_id: player.id(),
//                         action: Action::Inventory(player.inventory()),
//                     })
//                     .await;
//             }
//             Action::ConnectNbr => {
//                 todo!("Implement connect_nbr")
//             }
//             Action::Fork => {
//                 todo!("Implement fork")
//             }
//             Action::Eject => {
//                 todo!("Implement eject")
//             }
//             Action::Take(_) => {
//                 todo!("Implement take")
//             }
//             Action::Set(_) => {
//                 todo!("Implement set")
//             }
//             Action::Incantation => {
//                 todo!("Implement incantation")
//             }
//             Action::Disconnect => {
//                 self.pending_clients.remove(&action.client_id); // ensure client is removed
//                 self.clients.remove(&action.client_id); // ensure client is removed
//                 println!("{:?}", self.pending_clients);
//                 println!("{:?}", self.clients);
//             }
//             Action::Login(team_name) => {
//                 //todo! gui team
//                 let pending_client = self.pending_clients.remove(&action.client_id);
//                 let Some(mut pending_client) = pending_client else {
//                     //client disconnected in between
//                     return;
//                 };
//                 let team = self
//                     .teams
//                     .values_mut()
//                     .find(|team| team.name() == team_name);
//                 let Some(team) = team else {
//                     pending_client.ko().await;
//                     self.pending_clients
//                         .insert(action.client_id, pending_client);
//                     return;
//                 };
//                 let player = Player::new(team.id(), pending_client);
//                 self.add_player(player).await;
//             }
//         }
