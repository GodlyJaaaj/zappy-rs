use crate::cell::Cell;
use crate::resources::{Resource, Resources};
use crate::vec2::{Position, Size};
use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Map {
    size: Size,
    map: Vec<Vec<Cell>>,
    resources: Resources,
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
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
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
