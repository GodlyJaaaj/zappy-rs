use crate::cell::Cell;
use crate::vec2::Size;
use std::fmt;

pub struct Map {
    size: Size,
    map: Vec<Vec<Cell>>,
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
