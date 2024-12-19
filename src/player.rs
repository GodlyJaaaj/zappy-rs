use crate::pending::PendingClient;
use crate::protocol;
use crate::protocol::{ClientAction, Ko};
use crate::resources::Resources;
use crate::vec2::Position;
use rand::random;
use tokio::sync::mpsc;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthWest,
    NorthWest,
    SouthEast,
}

impl Direction {
    pub(crate) fn to_radians(&self) -> f64 {
        match self {
            Direction::North => 0.0,
            Direction::East => std::f64::consts::FRAC_PI_2,
            Direction::South => std::f64::consts::PI,
            Direction::West => std::f64::consts::PI + std::f64::consts::FRAC_PI_2,
            Direction::NorthEast => std::f64::consts::FRAC_PI_4,
            Direction::SouthWest => std::f64::consts::PI + std::f64::consts::FRAC_PI_4,
            Direction::NorthWest => std::f64::consts::PI + std::f64::consts::FRAC_PI_4,
            Direction::SouthEast => std::f64::consts::FRAC_PI_4,
        }
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
}

#[derive(Clone, Debug)]
pub struct Player {
    team: String,
    id: u64,
    inventory: Resources,
    pos: Position,
    direction: Direction,
    elevation: u8,
    satiety: u8,
    client_tx: mpsc::Sender<ClientAction>,
}

impl Player {
    pub fn new(team: String, pending_client: PendingClient) -> Self {
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
