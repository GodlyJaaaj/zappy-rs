use crate::player::Direction;
use crate::resources::{ElevationLevel, Resource, Resources};
use crate::vec2::{Size, UPosition};
use log::error;
use std::str::FromStr;
use std::sync::Arc;

pub type Id = u64;

pub trait HasId {
    fn id(&self) -> Id;
}

#[derive(Debug)]
pub enum SharedAction {
    Disconnected,
    InvalidAction,
    InvalidParameters,
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
    Msz,
    Bct(UPosition),
    Mct,
    Tna,
    Ppo(Id),
    Plv(Id),
    Pin(Id),
    Sgt,
    Sst(u64),
}

#[derive(Debug)]
pub enum PendingAction {
    Shared(SharedAction),
    Login(String),
}

pub(crate) type LookResult = Vec<(u64, Resources)>; // u64 = how many players on this cell

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

pub type BctResponse = (UPosition, Resources);

#[derive(Debug)]
pub enum GUIResponse {
    Shared(SharedResponse),
    Sbp,

    Msz(UPosition),
    Bct(BctResponse),
    Mct(Vec<BctResponse>),
    Tna(Vec<String>),
    Pnw(Id, UPosition, Direction, ElevationLevel, String),
    Ppo(Id, UPosition, Direction),
    Plv(Id, ElevationLevel),
    Pin(Id, UPosition, Resources),
    Pex(Id),
    Pbc(Id, Arc<String>),
    Pic(UPosition, ElevationLevel, Vec<Id>),
    Pie(UPosition, bool),
    Pfk(Id),
    Pdr(Id, Resource),
    Pgt(Id, Resource),
    Pdi(Id),
    Enw(Id, Id, UPosition),
    Ebo(Id),
    Edi(Id),
    Sgt(u64),
    Sst(u64),
    Seg(String),
    Smg(Arc<String>),
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
    Gui(GUIResponse),
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

pub fn parse_prefixed_id<T: FromStr>(input: &str, prefix: char) -> Option<T> {
    let cleaned = input.trim().strip_prefix(prefix).unwrap_or(input);
    cleaned.parse::<T>().ok()
}
