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
pub enum Action {
    //Game Action
    Broadcast(String),
    Forward(u64),
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
    //GUI Action
}
