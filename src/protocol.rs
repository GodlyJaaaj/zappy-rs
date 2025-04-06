use crate::resources::Resources;
use crate::vec2::Size;
use log::error;
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

#[derive(Debug)]
pub enum AIResponse {
    Shared(SharedResponse),
    Dead,
    Broadcast(u8, Arc<String>),
    Inventory(Resources),
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
