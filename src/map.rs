use crate::cell::Cell;
use crate::egg::Egg;
use crate::protocol::{HasId, Id};
use crate::resources::{Resource, Resources};
use crate::vec2::{Position, Size};
use rand::Rng;
use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Map {
    size: Size,
    map: Vec<Vec<Cell>>,
    resources: Resources,
    eggs: Vec<Egg>,
}

impl Index<Position> for Map {
    type Output = Cell;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.map[pos.y as usize][pos.x as usize]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.map[pos.y as usize][pos.x as usize]
    }
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

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn nb_eggs_by_team(&self, team_id: Id) -> u64 {
        self.eggs.iter().filter(|egg| egg.id() == team_id).count() as u64
    }

    pub fn spawn_egg(&mut self, team_id: Id, pos: Position) {
        let new_egg = Egg::new(team_id, pos);
        self.eggs.push(new_egg);
    }

    pub fn spawn_eggs(&mut self, team_id: Id, amount: u64) {
        (0..amount).for_each(|_| {
            let x = rand::rng().random_range(0..self.size.x());
            let y = rand::rng().random_range(0..self.size.y());
            let pos = Position::new(x, y);
            self.spawn_egg(team_id, pos);
        });
    }

    pub fn drop_egg(&mut self, team_id: Id) -> Option<Egg> {
        if let Some(pos) = self.eggs.iter().position(|egg| egg.id() == team_id) {
            let egg = self.eggs.remove(pos);
            Some(egg)
        } else {
            None
        }
    }

    pub fn add_resource(&mut self, resource: Resource, amount: u64, pos: Position) {
        self.resources[resource] += amount;
        self[pos].add_resource(resource, amount);
    }

    pub fn del_resource(
        &mut self,
        resource: Resource,
        amount: u64,
        pos: Position,
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
