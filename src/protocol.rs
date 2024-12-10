use crate::resources::Resource;

#[derive(Debug)]
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
