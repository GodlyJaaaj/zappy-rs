use core::ops::{Index, IndexMut};
use std::fmt;

#[derive(Debug, PartialEq)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Resource {
    Deraumere,
    Linemate,
    Mendiane,
    Phiras,
    Sibur,
    Thystame,
    Food, // Keep this last.
}

impl Resource {
    pub fn iter() -> impl Iterator<Item = Resource> {
        [
            Resource::Deraumere,
            Resource::Linemate,
            Resource::Mendiane,
            Resource::Phiras,
            Resource::Sibur,
            Resource::Thystame,
            Resource::Food,
        ]
        .iter()
        .cloned()
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Resources {
    contents: [u64; Resource::Food as usize + 1],
}

impl Index<Resource> for Resources {
    type Output = u64;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.contents[index as usize]
    }
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self.contents[index as usize]
    }
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{}",
            self[Resource::Deraumere],
            self[Resource::Linemate],
            self[Resource::Mendiane],
            self[Resource::Phiras],
            self[Resource::Sibur],
            self[Resource::Thystame],
            self[Resource::Food]
        )
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
