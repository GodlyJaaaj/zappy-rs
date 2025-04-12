use crate::pending::PendingClient;
use crate::protocol::{ClientSender, HasId, Id, ServerResponse};
use crate::resources::{ElevationLevel, Resource, Resources};
use crate::vec2::{HasPosition, Position, Size, UPosition};
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelativeDirection {
    Back,
    Left,
    Front,
    Right,
}

impl From<RelativeDirection> for u8 {
    fn from(dir: RelativeDirection) -> Self {
        match dir {
            RelativeDirection::Back => 5,
            RelativeDirection::Left => 3,
            RelativeDirection::Front => 1,
            RelativeDirection::Right => 7,
        }
    }
}

impl From<Direction> for i8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => 1,
            Direction::East => 2,
            Direction::South => 3,
            Direction::West => 4,
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
    team: Id,
    id: Id,
    inventory: Resources,
    pos: UPosition,
    direction: Direction,
    elevation: ElevationLevel,
    satiety: u64,
    client_tx: Sender<ServerResponse>,
    state: PlayerState,
}

impl Player {
    pub fn is_incantating(&self) -> bool {
        self.state == PlayerState::Incantating
    }
    pub fn level(&self) -> ElevationLevel {
        self.elevation
    }

    pub fn level_mut(&mut self) -> &mut ElevationLevel {
        &mut self.elevation
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn state_mut(&mut self) -> &mut PlayerState {
        &mut self.state
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

    pub fn team_id(&self) -> Id {
        self.team
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

    pub fn get_visible_positions(&self) -> Vec<Position> {
        let mut visible_positions = Vec::new();

        visible_positions.push(Position::new(self.pos.x as i64, self.pos.y as i64));
        for y in 1..=self.elevation as u8 + 1 {
            for x in -(y as i64)..=(y as i64) {
                let rel_pos = match self.direction() {
                    Direction::North => Position::new(x, y as i64),
                    Direction::East => Position::new(y as i64, -x),
                    Direction::South => Position::new(-x, -(y as i64)),
                    Direction::West => Position::new(-(y as i64), x),
                };
                let abs_pos = Position::new(
                    self.position().x as i64 + rel_pos.x,
                    self.position().y as i64 + rel_pos.y,
                );

                visible_positions.push(abs_pos);
            }
        }

        visible_positions
    }
}

impl HasPosition for Player {
    fn position(&self) -> UPosition {
        self.pos
    }

    fn position_mut(&mut self) -> &mut UPosition {
        &mut self.pos
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

pub struct PlayerBuilder {
    team: Option<Id>,
    id: Option<Id>,
    inventory: Resources,
    pos: UPosition,
    direction: Direction,
    elevation: ElevationLevel,
    satiety: u64,
    client_tx: Option<Sender<ServerResponse>>,
    state: PlayerState,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        PlayerBuilder {
            team: None,
            id: None,
            inventory: Resources::builder().food(10).build(),
            pos: UPosition::default(),
            direction: Direction::default(),
            elevation: ElevationLevel::default(),
            satiety: REFILL_PER_FOOD,
            client_tx: None,
            state: PlayerState::default(),
        }
    }

    pub fn team(mut self, team: Id) -> Self {
        self.team = Some(team);
        self
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn inventory(mut self, inventory: Resources) -> Self {
        self.inventory = inventory;
        self
    }

    pub fn position(mut self, pos: UPosition) -> Self {
        self.pos = pos;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn elevation(mut self, elevation: ElevationLevel) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn satiety(mut self, satiety: u64) -> Self {
        self.satiety = satiety;
        self
    }

    pub fn client_tx(mut self, client_tx: Sender<ServerResponse>) -> Self {
        self.client_tx = Some(client_tx);
        self
    }

    pub fn pending_client(mut self, pending_client: PendingClient) -> Self {
        self.id = Some(pending_client.id());
        self.client_tx = Some(pending_client.client_tx);
        self
    }

    pub fn state(mut self, state: PlayerState) -> Self {
        self.state = state;
        self
    }

    pub fn build(self) -> Result<Player, &'static str> {
        let team = self.team.ok_or("Team ID is required")?;
        let id = self.id.ok_or("Player ID is required")?;
        let client_tx = self.client_tx.ok_or("Client transmitter is required")?;

        Ok(Player {
            team,
            id,
            inventory: self.inventory,
            pos: self.pos,
            direction: self.direction,
            elevation: self.elevation,
            satiety: self.satiety,
            client_tx,
            state: self.state,
        })
    }
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }

    pub fn new(team: Id, pending_client: PendingClient) -> Self {
        PlayerBuilder::new()
            .team(team)
            .pending_client(pending_client)
            .build()
            .expect("Failed to build Player with valid parameters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ElevationLevel::Level2;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_player_builder() {
        let (tx, _rx) = mpsc::channel(10);
        let player = PlayerBuilder::new()
            .team(42)
            .id(1)
            .client_tx(tx)
            .position(UPosition::new(10, 20))
            .direction(Direction::North)
            .elevation(Level2)
            .satiety(200)
            .state(PlayerState::Idle)
            .build()
            .unwrap();

        assert_eq!(player.id(), 1);
        assert_eq!(player.position(), UPosition::new(10, 20));
        assert_eq!(player.direction(), Direction::North);
        assert_eq!(player.state(), PlayerState::Idle);
    }

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
