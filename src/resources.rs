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
        .into_iter()
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Resources {
    contents: [u64; Resource::Food as usize + 1],
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> ResourcesBuilder {
        ResourcesBuilder::new()
    }
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

pub struct InventoryFormat<'a>(pub &'a Resources);

impl<'a> fmt::Display for InventoryFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.0;
        write!(
            f,
            "[deraumere {}, linemate {}, mendiane {}, phiras {}, sibur {}, thystame {}, food {}]",
            r[Resource::Deraumere],
            r[Resource::Linemate],
            r[Resource::Mendiane],
            r[Resource::Phiras],
            r[Resource::Sibur],
            r[Resource::Thystame],
            r[Resource::Food]
        )
    }
}

pub struct ResourcesBuilder {
    resources: Resources,
}

impl ResourcesBuilder {
    pub fn new() -> Self {
        Self {
            resources: Resources::default(),
        }
    }
    pub fn deraumere(mut self, amount: u64) -> Self {
        self.resources[Resource::Deraumere] = amount;
        self
    }

    pub fn linemate(mut self, amount: u64) -> Self {
        self.resources[Resource::Linemate] = amount;
        self
    }

    pub fn mendiane(mut self, amount: u64) -> Self {
        self.resources[Resource::Mendiane] = amount;
        self
    }

    pub fn phiras(mut self, amount: u64) -> Self {
        self.resources[Resource::Phiras] = amount;
        self
    }

    pub fn sibur(mut self, amount: u64) -> Self {
        self.resources[Resource::Sibur] = amount;
        self
    }

    pub fn thystame(mut self, amount: u64) -> Self {
        self.resources[Resource::Thystame] = amount;
        self
    }

    pub fn food(mut self, amount: u64) -> Self {
        self.resources[Resource::Food] = amount;
        self
    }

    pub fn resource(mut self, resource: Resource, amount: u64) -> Self {
        self.resources[resource] = amount;
        self
    }

    pub fn build(self) -> Resources {
        self.resources
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resources_builder() {
        let resources = Resources::builder()
            .deraumere(5)
            .linemate(3)
            .food(10)
            .build();

        assert_eq!(resources[Resource::Deraumere], 5);
        assert_eq!(resources[Resource::Linemate], 3);
        assert_eq!(resources[Resource::Food], 10);
        assert_eq!(resources[Resource::Mendiane], 0);
    }
}
