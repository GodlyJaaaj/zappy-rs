use crate::cell::Cell;
use crate::vec2::Size;
use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Map {
    size: Size,
    map: Vec<Vec<Cell>>,
}

impl Index<(u64, u64)> for Map {
    type Output = Cell;

    fn index(&self, index: (u64, u64)) -> &Self::Output {
        &self.map[index.1 as usize][index.0 as usize]
    }
}

impl IndexMut<(u64, u64)> for Map {
    fn index_mut(&mut self, index: (u64, u64)) -> &mut Self::Output {
        &mut self.map[index.1 as usize][index.0 as usize]
    }
}

impl Map {
    pub fn new(size: Size) -> Self {
        Map {
            size,
            map: vec![vec![Cell::new(); size.x() as usize]; size.y() as usize],
        }
    }

    pub fn size(&self) -> Size {
        self.size
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
