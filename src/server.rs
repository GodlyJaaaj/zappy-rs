use crate::connection::Connection;
use crate::constant::{RELATIVE_DIRECTIONS, SATIETY_LOSS_PER_TICK};
use crate::event::Event;
use crate::event::EventScheduler;
use crate::gui::{Gui, GuiBuilder};
use crate::map::Map;
use crate::pending::PendingClient;
use crate::player::{Direction, Player, PlayerState};
use crate::protocol::PendingResponse::{LogAs, Shared};
use crate::protocol::{
    AIAction, AIResponse, BctResponse, ClientSender, EventType, GUIAction, GUIResponse, GameEvent,
    HasId, Id, PendingAction, ServerResponse, SharedAction, SharedResponse, TeamType,
};
use crate::resources::{Resource, Resources, LEVEL_REQUIREMENTS};
use crate::sound::get_sound_direction;
use crate::team::Team;
use crate::vec2::{HasPosition, Position, Size, UPosition};
use log::{debug, info, warn};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio::{select, time};

#[derive(Debug)]
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
    map: Map,
    teams: HashMap<Id, Team>,
    pending_clients: HashMap<Id, PendingClient>,
    clients: HashMap<Id, Player>,
    guis: HashMap<Id, Gui>,
    event_scheduler: EventScheduler<Event>,
    last_gui_notify: Instant,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("socket error: {0}")]
    FailedToBind(#[from] std::io::Error),
}

impl Server {
    pub async fn from_config(config: ServerConfig) -> Result<Server, ServerError> {
        let addr = format!("{}:{}", config.addr, config.port);
        debug!("Server using config {:?}", config);
        let socket = TcpListener::bind(&addr).await?;
        let (tx, rx) = mpsc::channel::<EventType>(32);
        let tick_interval = time::interval(time::Duration::from_nanos(
            (1_000_000_000f64 / config.freq as f64) as u64,
        ));

        let mut teams: HashMap<Id, Team> = HashMap::new();

        for (team_id, team_name) in config.teams.into_iter().enumerate() {
            if team_name == "GRAPHIC" {
                warn!("'GRAPHIC' can't be used as a team name and will be ignored");
                continue;
            }
            teams.insert(
                team_id as Id,
                Team::new(
                    team_id as Id,
                    team_name
                        .replace("\n", "_")
                        .replace("\r", "_")
                        .replace(" ", ""),
                ),
            );
        }

        let mut map = Map::new(Size::new(config.width as u64, config.height as u64));

        for (team_id, ..) in &teams {
            map.spawn_eggs(*team_id, config.clients_nb);
        }

        Ok(Server {
            global_channel: ThreadChannel { tx, rx },
            tick_interval,
            socket,
            map,
            teams,
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            guis: HashMap::new(),
            event_scheduler: EventScheduler::new(),
            last_gui_notify: Instant::now(),
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
                let pos = UPosition::new(x, y);
                self.map.add_resource(res, 1, pos, &mut self.guis);
            });
        }
    }

    fn set_tick_interval(&mut self, freq: u16) {
        let freq = (1_000_000_000f64 / freq as f64) as u64;
        self.tick_interval = time::interval(time::Duration::from_nanos(freq));
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

    fn accept_client(&mut self, socket: TcpStream, _: SocketAddr) {
        static CLIENT_ID: AtomicU64 = AtomicU64::new(0);
        let client_id: Id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
        info!(
            "Accepted connection from {:?} with id {}",
            socket.peer_addr().unwrap(),
            client_id
        );
        let server_tx = self.global_channel.tx.clone();
        let (client_tx, client_rx) = mpsc::channel::<ServerResponse>(256);
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
        self.event_scheduler.display_pending_events();
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
                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Pbc(
                            emitter.id(),
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
                    emitter
                        .move_forward(&self.map.size())
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Ppo(emitter.id(), emitter.position(), emitter.direction())));
                    }
                }
                Event::Right => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    emitter.direction_mut().rotate_right();
                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Ppo(emitter.id(), emitter.position(), emitter.direction())));
                    }
                }
                Event::Left => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    emitter.direction_mut().rotate_left();
                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));

                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Ppo(emitter.id(), emitter.position(), emitter.direction())));
                    }
                }
                Event::Look => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    let visible_pos = emitter.get_visible_positions();
                    let mut res = vec![];
                    for cell_pos in visible_pos {
                        let converted_pos = self.map.get_pos_signed(cell_pos);
                        let nb_players_on_cell = self
                            .clients
                            .values()
                            .filter(|client| client.position() == converted_pos)
                            .count();
                        let resources_on_cell =
                            self.map.get_ressources_at_pos(converted_pos).clone();
                        res.push((nb_players_on_cell as u64, resources_on_cell));
                    }
                    self.clients
                        .get_mut(&timed_event.player_id)
                        .unwrap()
                        .send_to_client(ServerResponse::AI(AIResponse::Look(res)));
                }
                Event::Inventory => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    emitter.send_to_client(ServerResponse::AI(AIResponse::Inventory(
                        emitter.inventory(),
                    )));
                }
                Event::ConnectNbr => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    emitter.send_to_client(ServerResponse::AI(AIResponse::ConnectNbr(
                        self.map.nb_eggs_by_team(emitter.team_id()),
                    )));
                }
                Event::Fork => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    let egg_id = self.map.spawn_egg(emitter.team_id(), emitter.position());
                    //todo egg hatching ? 600 ticks ?

                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Pfk(emitter.id())));
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Enw(
                            egg_id,
                            emitter.id(),
                            emitter.position(),
                        )));
                    }

                    emitter
                        .send_to_client(ServerResponse::AI(AIResponse::Shared(SharedResponse::Ok)));
                }
                Event::Eject => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };

                    let (pusher_pos, pusher_direction, pusher_id) =
                        (emitter.position(), emitter.direction(), emitter.id());

                    let players_on_same_pos: Vec<_> = self
                        .clients
                        .iter_mut()
                        .filter_map(|(_, player)| {
                            if player.position() == pusher_pos && player.id() != pusher_id {
                                Some(player)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let offset = match pusher_direction {
                        Direction::North => (0, 1),
                        Direction::East => (1, 0),
                        Direction::South => (0, -1),
                        Direction::West => (-1, 0),
                    };
                    let nb_pushed_players = players_on_same_pos.len();
                    let new_pos = self
                        .map
                        .get_pos_with_offset(pusher_pos, Position::new(offset.0, offset.1));
                    let direction: i8 = pusher_direction.into();
                    for player in players_on_same_pos {
                        player.position_mut().replace(new_pos);
                        let pushed_dir: i8 = player.direction().into();
                        let res = (direction - pushed_dir + 4).rem_euclid(4);
                        let res = RELATIVE_DIRECTIONS[res as usize];
                        //gui
                        for (.., gui) in &self.guis {
                            gui.send_to_client(ServerResponse::Gui(GUIResponse::Ppo(player.id(), player.position(), player.direction())));
                        }
                        player.send_to_client(ServerResponse::AI(AIResponse::Eject(res.into())));
                    }
                    let broken_eggs = self.map.break_eggs_at_pos(pusher_pos);
                    let emitter = self.clients.get_mut(&timed_event.player_id).unwrap(); //safe since we know the player exists
                    if nb_pushed_players > 0 || !broken_eggs.is_empty() {
                        debug!(
                            "Client {} broke {} eggs and pushed {} players",
                            emitter.id(),
                            broken_eggs.len(),
                            nb_pushed_players
                        );
                        //gui
                        for (.., gui) in &self.guis {
                            gui.send_to_client(ServerResponse::Gui(GUIResponse::Pex(emitter.id())));
                            for broken_egg in &broken_eggs {
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Edi(
                                    broken_egg.id(),
                                )));
                            }
                        }

                        emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                            SharedResponse::Ok,
                        )));
                    } else {
                        emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                            SharedResponse::Ko,
                        )));
                    }
                }
                Event::Take(resource) => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    match self.map.del_resource(resource, 1, emitter.position(), &mut self.guis) {
                        None => {
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ko,
                            )));
                        }
                        Some(_) => {
                            //gui
                            for (.., gui) in &self.guis {
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Pgt(
                                    emitter.id(),
                                    resource,
                                )));
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Pin(
                                    emitter.id(),
                                    emitter.position(),
                                    emitter.inventory(),
                                )));
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Bct((
                                    emitter.position(),
                                    self.map[emitter.position()].ressources().clone(),
                                ))));
                            }

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
                    let res = emitter.del_resource(resource, 1);
                    match res {
                        None => {
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ko,
                            )));
                        }
                        Some(resource) => {
                            self.map.add_resource(resource, 1, emitter.position(), &mut self.guis);

                            //gui
                            for (.., gui) in &self.guis {
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Pdr(
                                    emitter.id(),
                                    resource,
                                )));
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Pin(
                                    emitter.id(),
                                    emitter.position(),
                                    emitter.inventory(),
                                )));
                                gui.send_to_client(ServerResponse::Gui(GUIResponse::Bct((
                                    emitter.position(),
                                    self.map[emitter.position()].ressources().clone(),
                                ))));
                            }
                            emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                SharedResponse::Ok,
                            )));
                        }
                    }
                }
                Event::Incantation => {
                    let Some(emitter) = self.clients.get_mut(&timed_event.player_id) else {
                        continue;
                    };
                    let emitter_pos = emitter.position();
                    let emitter_level = emitter.level();
                    let emitter_id = emitter.id();
                    debug!(
                        "Incantation requirements for Client {}: {:?}",
                        emitter.id(),
                        LEVEL_REQUIREMENTS[&emitter_level]
                    );
                    let players_on_tile: Vec<Id> = self
                        .clients
                        .iter()
                        .filter_map(|(id, player)| {
                            if player.position() == emitter_pos
                                && !player.is_incantating()
                                && player.level() == emitter_level
                            {
                                Some(*id)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let resources_on_tile: &Resources = self.map.get_ressources_at_pos(emitter_pos);
                    let requirement = &LEVEL_REQUIREMENTS[&emitter_level];

                    if players_on_tile.len() < requirement.needed_players()
                        || !resources_on_tile.has_at_least(requirement.needed_resources())
                    {
                        let emitter = self.clients.get_mut(&timed_event.player_id).unwrap();
                        emitter.send_to_client(ServerResponse::AI(AIResponse::Shared(
                            SharedResponse::Ko,
                        )));
                        return;
                    }

                    for id in &players_on_tile {
                        let player = self.clients.get_mut(id).unwrap();
                        *player.state_mut() = PlayerState::Incantating;
                        player.send_to_client(ServerResponse::AI(AIResponse::Incantating));
                        if *id != emitter_id {
                            self.event_scheduler.shift_client_events(*id, 300);
                            self.event_scheduler
                                .force_schedule(Event::Phantom, 300, *id);
                        }
                        println!("Player {} is now {:?}", id, player.state_mut());
                    }

                    let emitter = self.clients.get_mut(&timed_event.player_id).unwrap();

                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Pic(
                            emitter_pos,
                            emitter.level(),
                            players_on_tile.clone(),
                        )));
                    }

                    let new_event =
                        Event::IncantationEnd(players_on_tile, requirement, emitter.position());
                    self.event_scheduler.schedule(new_event, 300, emitter.id());
                }
                Event::IncantationEnd(players_incantating, requirement, incantation_pos) => {
                    let mut players_still_on_tile: Vec<Id> = vec![];

                    for id in &players_incantating {
                        if let Some(player) = self.clients.get_mut(id) {
                            if player.is_incantating() && player.position() == incantation_pos {
                                *player.state_mut() = PlayerState::Idle;
                                players_still_on_tile.push(*id);
                            }
                        }
                    }

                    let resources_on_tile: &Resources =
                        self.map.get_ressources_at_pos(incantation_pos);

                    if players_still_on_tile.len() < requirement.needed_players()
                        || !resources_on_tile.has_at_least(requirement.needed_resources())
                    {
                        //gui
                        for (.., gui) in &self.guis {
                            gui.send_to_client(ServerResponse::Gui(GUIResponse::Pie(
                                incantation_pos,
                                false,
                            )));
                        }

                        for id in &players_incantating {
                            if let Some(client) = self.clients.get_mut(id) {
                                client.send_to_client(ServerResponse::AI(AIResponse::Shared(
                                    SharedResponse::Ko,
                                )));
                            }
                        }
                        return;
                    }
                    for resource_type in Resource::iter() {
                        let amount = requirement.needed_resources()[resource_type];
                        if amount > 0 {
                            self.map
                                .del_resource(resource_type, amount, incantation_pos, &mut self.guis);
                        }
                    }
                    for id in &players_still_on_tile {
                        let client = self.clients.get_mut(id).unwrap();
                        *client.level_mut() = client.level().upgrade();
                        client.send_to_client(ServerResponse::AI(AIResponse::LevelUp(
                            client.level(),
                        )));

                        //gui
                        for (.., gui) in &self.guis {
                            gui.send_to_client(ServerResponse::Gui(GUIResponse::Plv(
                                client.id(),
                                client.level(),
                            )));
                        }
                    }

                    //gui
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Pie(
                            incantation_pos,
                            true,
                        )));
                    }
                    debug!(
                        "Incantation successful for Clients : {:?}",
                        players_still_on_tile
                    );
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
                Event::Phantom => continue,
            }
        }
        self.reduce_satiety();
    }

    pub fn reduce_satiety(&mut self) {
        for (id, client) in self.clients.iter_mut() {
            if client.reduce_satiety(SATIETY_LOSS_PER_TICK) == 0 {
                client.send_to_client(ServerResponse::AI(AIResponse::Dead));
                info!("Client {} is dead", id);
            }
        }

        // Notify GUIs if at least 1 second passed
        if self.last_gui_notify.elapsed() >= Duration::from_secs(1) {
            self.last_gui_notify = Instant::now();

            for client in self.clients.values() {
                for (.., gui) in &self.guis {
                    gui.send_to_client(ServerResponse::Gui(GUIResponse::Pin(
                        client.id(),
                        client.position(),
                        client.inventory(),
                    )));
                }
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
                self.handle_gui_events((id, action)).await;
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
            PendingAction::Shared(
                SharedAction::InvalidAction | SharedAction::InvalidParameters,
            ) => unreachable!(),
            PendingAction::Shared(SharedAction::ReachedTakeLimit) => {
                warn!("Pending client: {} sent too much data", id);
                send_ko(client);
            }
            PendingAction::Shared(SharedAction::InvalidEncoding) => {
                warn!("Pending client: {} uses invalid encoding", id);
                send_ko(client);
            }
            PendingAction::Login(team_name) => {
                if team_name == "GRAPHIC" {
                    let pending_client = self.pending_clients.remove(&id).unwrap();

                    let new_gui = GuiBuilder::new()
                        .pending_client(pending_client)
                        .build()
                        .unwrap();
                    new_gui.send_to_client(ServerResponse::Pending(LogAs(TeamType::Graphic)));
                    self.guis.insert(id, new_gui);
                    return;
                }

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
                    self.map.size(),
                ))));

                // gui
                for (.., gui) in &self.guis {
                    gui.send_to_client(ServerResponse::Gui(GUIResponse::Pnw(
                        player.id(),
                        player.position(),
                        player.direction(),
                        player.level(),
                        team_name.clone(),
                    )));
                    gui.send_to_client(ServerResponse::Gui(GUIResponse::Ebo(egg.id())));
                }

                self.clients.insert(player.id(), player);
            }
        }
    }

    async fn handle_ai_events(&mut self, (id, action): (Id, AIAction)) {
        match action {
            AIAction::Shared(shared) => match shared {
                SharedAction::Disconnected => {
                    for (.., gui) in &self.guis {
                        gui.send_to_client(ServerResponse::Gui(GUIResponse::Pdi(id)));
                    }
                    self.clients.remove(&id);
                }
                SharedAction::InvalidAction
                | SharedAction::ReachedTakeLimit
                | SharedAction::InvalidEncoding
                | SharedAction::InvalidParameters => {
                    self.event_scheduler.schedule(Event::Ko, 0, id);
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
                    self.event_scheduler.schedule(event, 0, id);
                }
                _ => {
                    unreachable!()
                }
            },
        }
    }

    async fn handle_gui_events(&mut self, (id, action): (Id, GUIAction)) {
        match action {
            GUIAction::Shared(shared) => match shared {
                SharedAction::Disconnected => {
                    self.guis.remove(&id);
                }
                SharedAction::InvalidAction
                | SharedAction::ReachedTakeLimit
                | SharedAction::InvalidEncoding => {
                    if let Some(emitter) = self.guis.get_mut(&id) {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Shared(
                            SharedResponse::Ko,
                        )));
                    }
                }
                SharedAction::InvalidParameters => {
                    if let Some(emitter) = self.guis.get_mut(&id) {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sbp));
                    }
                }
            },
            GUIAction::Msz => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Msz(self.map.size())));
                }
            }
            GUIAction::Bct(pos) => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    let Some(cell) = self.map.get(pos) else {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sbp));
                        return;
                    };
                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Bct((
                        pos,
                        cell.ressources().clone(),
                    ))));
                }
            }
            GUIAction::Mct => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    let bct_responses: Vec<BctResponse> = self
                        .map
                        .cells_with_positions()
                        .map(|(pos, cell)| (pos, cell.ressources().clone()))
                        .collect();

                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Mct(bct_responses)));
                }
            }
            GUIAction::Tna => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    let team_name = self
                        .teams
                        .iter()
                        .map(|(.., team_name)| team_name.name().to_string())
                        .collect::<Vec<_>>();
                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Tna(team_name)));
                }
            }
            GUIAction::Ppo(player_id) => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    if let Some(player) = self.clients.get(&player_id) {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Ppo(
                            player_id,
                            player.position(),
                            player.direction(),
                        )));
                    } else {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sbp));
                    }
                }
            }
            GUIAction::Plv(player_id) => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    if let Some(player) = self.clients.get(&player_id) {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Plv(
                            player_id,
                            player.level(),
                        )));
                    } else {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sbp));
                    }
                }
            }
            GUIAction::Pin(player_id) => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    if let Some(player) = self.clients.get(&player_id) {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Pin(
                            player_id,
                            player.position(),
                            player.inventory(),
                        )));
                    } else {
                        emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sbp));
                    }
                }
            }
            GUIAction::Sgt => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    let freq =
                        (1_000_000_000f64 / self.tick_interval.period().as_nanos() as f64) as u64;
                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sgt(freq)));
                }
            }
            GUIAction::Sst(freq) => {
                if let Some(emitter) = self.guis.get_mut(&id) {
                    let tick_interval = time::interval(time::Duration::from_nanos(
                        (1_000_000_000f64 / freq as f64) as u64,
                    ));
                    self.tick_interval = tick_interval;
                    emitter.send_to_client(ServerResponse::Gui(GUIResponse::Sst(freq)));
                }
            }
        }
    }
}
