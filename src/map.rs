use crate::cell::Cell;
use crate::egg::Egg;
use crate::protocol::{HasId, Id};
use crate::resources::{Resource, Resources};
use crate::vec2::{HasPosition, Position, Size, UPosition};
use rand::Rng;
use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Map {
    size: Size,
    map: Vec<Vec<Cell>>,
    resources: Resources,
    eggs: Vec<Egg>,
}

impl Index<UPosition> for Map {
    type Output = Cell;

    fn index(&self, pos: UPosition) -> &Self::Output {
        &self.map[pos.y as usize][pos.x as usize]
    }
}

impl IndexMut<UPosition> for Map {
    fn index_mut(&mut self, pos: UPosition) -> &mut Self::Output {
        &mut self.map[pos.y as usize][pos.x as usize]
    }
}

pub enum IncantationError {
    NotEnoughPlayers,
    NotEnoughRessources,
}

impl Map {
    pub fn new(size: Size) -> Self {
        Map {
            size,
            map: vec![vec![Cell::new(); size.x() as usize]; size.y() as usize],
            resources: Default::default(),
            eggs: Default::default(),
        }
    }

    pub fn get_pos(&self, pos: UPosition) -> UPosition {
        let wrapped_x = pos.x % self.size.x();
        let wrapped_y = pos.y % self.size.y();

        UPosition::new(wrapped_x, wrapped_y)
    }

    pub fn get_pos_with_offset(&self, pos: UPosition, offset: Position) -> UPosition {
        let new_x = (pos.x as i64 + offset.x).rem_euclid(self.size.x() as i64) as u64;
        let new_y = (pos.y as i64 + offset.y).rem_euclid(self.size.y() as i64) as u64;

        UPosition::new(new_x, new_y)
    }

    pub fn get_pos_signed(&self, pos: Position) -> UPosition {
        fn wrap(value: i64, max: u64) -> u64 {
            ((value % max as i64 + max as i64) % max as i64) as u64
        }

        let wrapped_x = wrap(pos.x, self.size.x());
        let wrapped_y = wrap(pos.y, self.size.y());

        UPosition::new(wrapped_x, wrapped_y)
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn get_ressources_at_pos(&self, pos: UPosition) -> &Resources {
        self[pos].ressources()
    }

    pub fn nb_eggs_by_team(&self, team_id: Id) -> u64 {
        self.eggs.iter().filter(|egg| egg.id() == team_id).count() as u64
    }

    pub fn spawn_egg(&mut self, team_id: Id, pos: UPosition) {
        let new_egg = Egg::new(team_id, pos);
        self.eggs.push(new_egg);
    }

    pub fn spawn_eggs(&mut self, team_id: Id, amount: u64) {
        (0..amount).for_each(|_| {
            let x = rand::rng().random_range(0..self.size.x());
            let y = rand::rng().random_range(0..self.size.y());
            let pos = UPosition::new(x, y);
            self.spawn_egg(team_id, pos);
        });
    }

    pub fn drop_egg(&mut self, team_id: Id) -> Option<Egg> {
        let egg_positions: Vec<usize> = self
            .eggs
            .iter()
            .enumerate()
            .filter_map(
                |(pos, egg)| {
                    if egg.id() == team_id { Some(pos) } else { None }
                },
            )
            .collect();

        if egg_positions.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        let random_index = rng.random_range(0..egg_positions.len());
        let position_to_remove = egg_positions[random_index];

        Some(self.eggs.remove(position_to_remove))
    }

    pub fn break_eggs_at_pos(&mut self, pos: UPosition) -> Vec<Egg> {
        let eggs_to_remove: Vec<usize> = self
            .eggs
            .iter()
            .enumerate()
            .filter_map(|(index, egg)| {
                if egg.position() == pos {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut removed_eggs = Vec::with_capacity(eggs_to_remove.len());
        for &index in eggs_to_remove.iter().rev() {
            removed_eggs.push(self.eggs.remove(index));
        }
        removed_eggs.reverse();
        removed_eggs
    }

    pub fn add_resource(&mut self, resource: Resource, amount: u64, pos: UPosition) {
        self.resources[resource] += amount;
        self[pos].add_resource(resource, amount);
    }

    pub fn del_resource(
        &mut self,
        resource: Resource,
        amount: u64,
        pos: UPosition,
    ) -> Option<Resource> {
        let res = self[pos].del_resource(resource, amount);
        if let Some(res) = res {
            self.resources[resource] -= amount;
            Some(res)
        } else {
            None
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
