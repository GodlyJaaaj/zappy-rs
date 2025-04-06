use crate::pending::PendingClient;
use crate::protocol::{ClientSender, HasId, Id, ServerResponse};
use crate::resources::{Resource, Resources};
use crate::vec2::{Position, Size};
use rand::random;
use tokio::sync::mpsc::Sender;

const REFILL_PER_FOOD: u64 = 126;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::new()
    }
}

impl Direction {
    fn new() -> Self {
        match random::<u8>() % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }

    pub fn rotate_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Incantating,
}

#[derive(Clone, Debug)]
pub struct Player {
    team: u64,
    id: u64,
    inventory: Resources,
    pos: Position,
    direction: Direction,
    elevation: u8,
    satiety: u64,
    client_tx: Sender<ServerResponse>,
    state: PlayerState,
}

impl Player {
    pub fn new(team: u64, pending_client: PendingClient) -> Self {
        Player {
            team,
            id: pending_client.id(),
            inventory: Resources::builder().food(10).build(),
            pos: Position::default(), //todo!
            direction: Direction::new(),
            elevation: 1,
            satiety: REFILL_PER_FOOD, // todo!
            client_tx: pending_client.client_tx,
            state: Default::default(),
        }
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn reduce_satiety(&mut self, reduction: u64) -> u64 {
        let new_satiety = self.satiety.saturating_sub(reduction);

        if new_satiety == 0 {
            if self.inventory[Resource::Food] > 0 {
                self.inventory[Resource::Food] = self.inventory[Resource::Food].saturating_sub(1);
                self.satiety = new_satiety.saturating_add(REFILL_PER_FOOD);
            } else {
                self.satiety = new_satiety;
            }
        } else {
            self.satiety = new_satiety;
        }
        self.satiety
    }

    pub fn inventory(&self) -> Resources {
        self.inventory.clone()
    }

    pub fn direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn pos_mut(&mut self) -> &mut Position {
        &mut self.pos
    }

    pub fn move_forward(&mut self, map_size: &Size) -> &mut Self {
        match self.direction {
            Direction::North => self.move_player(0, 1, map_size),
            Direction::East => self.move_player(1, 0, map_size),
            Direction::South => self.move_player(0, -1, map_size),
            Direction::West => self.move_player(-1, 0, map_size),
        }
    }

    pub fn add_resource(&mut self, resource: Resource, amount: u64) -> &mut Self {
        self.inventory[resource] += amount;
        self
    }

    pub fn del_resource(&mut self, resource: Resource, amount: u64) -> Option<Resource> {
        if self.inventory[resource] >= amount {
            self.inventory[resource] -= amount;
            Some(resource)
        } else {
            None
        }
    }

    pub fn move_player(&mut self, dx: isize, dy: isize, map_size: &Size) -> &mut Self {
        self.pos.x = (self.pos.x() as isize + dx).rem_euclid(map_size.x() as isize) as u64;
        self.pos.y = (self.pos.y() as isize + dy).rem_euclid(map_size.y() as isize) as u64;
        self
    }
}

impl HasId for Player {
    fn id(&self) -> Id {
        self.id
    }
}

impl ClientSender for Player {
    fn get_client_tx(&self) -> &Sender<ServerResponse> {
        &self.client_tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_direction_rotate_right() {
        let mut direction = Direction::North;
        direction.rotate_right();
        assert_eq!(direction, Direction::East);
    }

    #[tokio::test]
    async fn test_direction_rotate_left() {
        let mut direction = Direction::North;
        direction.rotate_left();
        assert_eq!(direction, Direction::West);
    }
}
