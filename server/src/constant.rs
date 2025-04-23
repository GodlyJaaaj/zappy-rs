use crate::player::RelativeDirection;

pub const REFILL_PER_FOOD: u64 = 126;
pub const SATIETY_LOSS_PER_TICK: u64 = 1;
pub const MAX_LINE_SIZE: usize = 8193;
pub const RELATIVE_DIRECTIONS: [RelativeDirection; 4] = [
    RelativeDirection::Back,
    RelativeDirection::Left,
    RelativeDirection::Front,
    RelativeDirection::Right,
];
