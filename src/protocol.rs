use crate::resources::Resource;
use crate::vec2::Size;
use std::sync::Arc;

#[derive(Debug)]
pub enum ParsingError {
    InvalidAction,
}

pub struct ClientAction {
    pub client_id: u64,
    pub action: Action,
}

#[derive(Debug, PartialEq)]
pub enum ClientType {
    GUI,
    AI,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Ok,
    Ko,

    //Game Action
    Broadcast(u8, Arc<String>),
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
    LoggedIn(ClientType, u64, Size), // (number of slots available, (width, height) of the map)

                                     //GUI Action
}

pub trait Ko {
    async fn ko(&mut self) -> bool;
}
