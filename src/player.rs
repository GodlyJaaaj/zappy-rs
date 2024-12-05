use crate::resources::Resources;
use crate::vec2::Position;

pub struct Player {
    id: u128,
    inventory: Resources,
    pos: Position,
    elevation: u8,
    satiety: u8,
}
