use crate::resources::Resource;

pub enum ClientAction {
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
