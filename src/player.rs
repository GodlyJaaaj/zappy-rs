use crate::resources::Resources;
use crate::vec2::Position;

#[derive(Clone, Debug)]
pub struct Player {
    id: u128,
    inventory: Resources,
    pos: Position,
    elevation: u8,
    satiety: u8,
    // add channel to make the main thread able to answer to the player
}
