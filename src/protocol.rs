use crate::resources::Resource;
use crate::vec2::Size;
use log::error;

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
    Broadcast(String),
    Forward,
    Right,
    Left,
    Look,
    Inventory,
    ConnectNbr,
    Fork,
    Eject,
    Take(Resource),
    Set(Resource),
    Incantation,
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
}

#[derive(Debug)]
pub enum GUIResponse {
    Shared(SharedResponse),
}

#[derive(Debug)]
pub enum TeamType {
    Graphic,
    IA(String, u64, Size),
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
pub struct Event<T> {
    pub id: Id,
    pub action: T,
}

pub type AIEvent = Event<AIAction>;
pub type GUIEvent = Event<GUIAction>;
pub type PendingEvent = Event<PendingAction>;

#[derive(Debug)]
pub enum EventType {
    AI(AIEvent),
    GUI(GUIEvent),
    Pending(PendingEvent),
}

pub trait ClientSender {
    fn get_client_tx(&self) -> &tokio::sync::mpsc::Sender<ServerResponse>;
    async fn send_to_client(&self, response: ServerResponse) {
        match self.get_client_tx().send(response).await {
            Ok(_) => {}
            Err(e) => {
                error!("failed to send response to client: {}", e);
            }
        };
    }
}
