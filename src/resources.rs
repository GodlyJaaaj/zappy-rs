use core::ops::{Index, IndexMut};
use std::fmt;

#[repr(u8)]
pub enum Resource {
    Deraumere,
    Linemate,
    Mendiane,
    Phiras,
    Sibur,
    Thystame,
    Food,			// Keep this last.
}

#[derive(Default, Clone)]
pub struct Resources {
    contents: [u64; Resource::Food as usize]
}

impl Index<Resource> for Resources {
    type Output = u64;

    fn index(&self, index: Resource) -> &Self::Output {
        return &self.contents[index as usize];
    }
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        return &mut self.contents[index as usize];
    }
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
	       "{},{},{},{},{},{},{}",
	       self[Resource::Deraumere],
	       self[Resource::Linemate],
	       self[Resource::Mendiane],
	       self[Resource::Phiras],
	       self[Resource::Sibur],
	       self[Resource::Thystame],
	       self[Resource::Food])
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", self)
    }
}
