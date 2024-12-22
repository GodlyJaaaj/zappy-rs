use crate::pending::PendingClient;
use crate::protocol;
use crate::protocol::{ClientAction, Ko};
use crate::resources::Resources;
use crate::vec2::{Position, Size};
use rand::random;
use tokio::sync::mpsc;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
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

#[derive(Clone, Debug)]
pub struct Player {
    team: u64,
    id: u64,
    inventory: Resources,
    pos: Position,
    direction: Direction,
    elevation: u8,
    satiety: u8,
    client_tx: mpsc::Sender<ClientAction>,
}

impl Player {
    pub fn new(team: u64, pending_client: PendingClient) -> Self {
        Player {
            team,
            id: pending_client.id(),
            inventory: Resources::default(),
            pos: (0, 0).into(), // todo!
            direction: Direction::new(),
            elevation: 1,
            satiety: 10, // todo!
            client_tx: pending_client.client_tx(),
        }
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

    pub fn move_forward(&mut self, map_size: &Size) {
        match self.direction {
            Direction::North => self.move_player(0, 1, map_size),
            Direction::East => self.move_player(1, 0, map_size),
            Direction::South => self.move_player(0, -1, map_size),
            Direction::West => self.move_player(-1, 0, map_size),
        }
    }

    pub fn move_player(&mut self, dx: isize, dy: isize, map_size: &Size) {
        self.pos.x = (self.pos.x() as isize + dx).rem_euclid(map_size.x() as isize) as u64;
        self.pos.y = (self.pos.y() as isize + dy).rem_euclid(map_size.y() as isize) as u64;
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub async fn send(&self, action: ClientAction) {
        let _ = self.client_tx.send(action).await;
    }
}

impl Ko for Player {
    async fn ko(&mut self) -> bool {
        self.client_tx
            .send(ClientAction {
                client_id: self.id(),
                action: protocol::Action::Ko,
            })
            .await
            .is_ok()
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
