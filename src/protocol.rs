use crate::resources::Resource;

#[derive(Debug)]
pub enum ParsingError {
    InvalidAction,
}

pub struct ClientAction {
    pub client_id: u64,
    pub action: Action,
}

#[derive(Debug)]
pub enum ClientType {
    GUI,
    AI,
}

#[derive(Debug)]
pub enum Action {
    Ko,

    //Game Action
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

    //Server Action
    Disconnect,
    Login(String),
    LoggedIn(ClientType, u64, (u8, u8)), // (number of slots available, (width, height) of the map)

    //GUI Action
}

pub trait Ko {
    async fn ko(&self) -> bool;
}
