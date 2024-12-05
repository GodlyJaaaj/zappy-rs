use crate::player::Player;
use crate::resources::Resources;

struct Cell {
    resources: Resources,
    players: Vec<Player>,
}
