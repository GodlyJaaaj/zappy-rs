use crate::resources::Resource::{Deraumere, Food, Linemate, Mendiane, Phiras, Sibur, Thystame};
use crate::resources::{ElevationLevel, Resources};
use crate::vec2::Size;
use log::error;
use std::fmt;
use std::sync::Arc;

pub type Id = u64;

pub trait HasId {
    fn id(&self) -> Id;
}

#[derive(Debug)]
pub enum SharedAction {
    Disconnected,
    InvalidAction,
    ReachedTakeLimit,
    InvalidEncoding,
}

#[derive(Debug)]
pub enum SharedResponse {
    Ko,
    Ok,
}

#[derive(Debug)]
pub enum AIAction {
    Shared(SharedAction),
    Action(crate::event::Event),
}

#[derive(Debug)]
pub enum GUIAction {
    Shared(SharedAction),
}

#[derive(Debug)]
pub enum PendingAction {
    Shared(SharedAction),
    Login(String),
}

type LookResult = Vec<(u64, Resources)>; // u64 = how many players on this cell

#[derive(Debug)]
pub enum AIResponse {
    Shared(SharedResponse),
    Dead,
    Broadcast(u8, Arc<String>),
    Incantating,
    LevelUp(ElevationLevel),
    Inventory(Resources),
    ConnectNbr(u64),
    Eject(u8),
    Look(LookResult),
}

pub struct LookFormat<'a>(pub &'a LookResult);

impl fmt::Display for LookFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cells = self.0;
        let mut formatted_cells = Vec::new();

        for (player_count, resources) in cells {
            let mut cell_elements = Vec::new();

            // Add players
            for _ in 0..*player_count {
                cell_elements.push("player".to_string());
            }

            // Add resources
            let resource_names = [
                ("food", Food),
                ("linemate", Linemate),
                ("deraumere", Deraumere),
                ("sibur", Sibur),
                ("mendiane", Mendiane),
                ("phiras", Phiras),
                ("thystame", Thystame),
            ];

            for &(name, index) in &resource_names {
                for _ in 0..resources[index] {
                    cell_elements.push(name.to_string());
                }
            }

            if !formatted_cells.is_empty() {
                cell_elements.insert(0, "".to_string());
            }

            formatted_cells.push(cell_elements.join(" "));
        }

        write!(f, "[{}]", formatted_cells.join(","))
    }
}

#[derive(Debug)]
pub enum GUIResponse {
    Shared(SharedResponse),
}

#[derive(Debug)]
pub enum TeamType {
    Graphic,
    IA(u64, Size),
}

#[derive(Debug)]
pub enum PendingResponse {
    Shared(SharedResponse),
    LogAs(TeamType),
}

#[derive(Debug)]
pub enum ServerResponse {
    AI(AIResponse),
    GUI(GUIResponse),
    Pending(PendingResponse),
}

#[derive(Debug)]
pub struct GameEvent<T> {
    pub id: Id,
    pub action: T,
}

pub type AIEvent = GameEvent<AIAction>;
pub type GUIEvent = GameEvent<GUIAction>;
pub type PendingEvent = GameEvent<PendingAction>;

#[derive(Debug)]
pub enum EventType {
    AI(AIEvent),
    GUI(GUIEvent),
    Pending(PendingEvent),
}

pub trait ClientSender {
    fn get_client_tx(&self) -> &tokio::sync::mpsc::Sender<ServerResponse>;
    fn send_to_client(&self, response: ServerResponse) -> &Self {
        match self.get_client_tx().try_send(response) {
            Ok(_) => {}
            Err(e) => {
                error!("failed to send response to client (channel closed?): {}", e);
            }
        };
        self
    }
}
