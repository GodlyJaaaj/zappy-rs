use crate::resources::ElevationLevel::*;
use crate::resources::Resource::{Deraumere, Food, Linemate, Mendiane, Phiras, Sibur, Thystame};
use core::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::sync::LazyLock;

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
            Deraumere,
            Linemate,
            Mendiane,
            Phiras,
            Sibur,
            Thystame,
            Food,
        ]
        .into_iter()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum ElevationLevel {
    Level0,
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

#[derive(Debug, Clone)]
pub struct LevelRequirement {
    players_needed: u64,  // Number of players needed
    resources: Resources, // Resources needed for the level
}

impl ElevationLevel {
    pub fn upgrade(self) -> ElevationLevel {
        match self {
            Level0 => Level1,
            Level1 => Level2,
            Level2 => Level3,
            Level3 => Level4,
            Level4 => Level5,
            Level5 => Level6,
            Level6 => Level7,
            Level7 => Level8,
            Level8 => Level8,
        }
    }
}

impl LevelRequirement {
    pub fn needed_resources(&self) -> &Resources {
        &self.resources
    }

    pub fn needed_players(&self) -> usize {
        self.players_needed as usize
    }
}

pub static LEVEL_REQUIREMENTS: LazyLock<HashMap<ElevationLevel, LevelRequirement>> =
    LazyLock::new(|| {
        let mut requirements = HashMap::new();

        requirements.insert(
            Level1,
            LevelRequirement {
                players_needed: 1,
                resources: Resources::builder().linemate(1).build(),
            },
        );

        requirements.insert(
            Level2,
            LevelRequirement {
                players_needed: 2,
                resources: Resources::builder()
                    .linemate(1)
                    .deraumere(1)
                    .sibur(1)
                    .build(),
            },
        );

        requirements.insert(
            Level3,
            LevelRequirement {
                players_needed: 2,
                resources: Resources::builder().linemate(2).sibur(1).phiras(2).build(),
            },
        );

        requirements.insert(
            Level4,
            LevelRequirement {
                players_needed: 4,
                resources: Resources::builder()
                    .linemate(1)
                    .deraumere(1)
                    .sibur(2)
                    .phiras(1)
                    .build(),
            },
        );
        requirements.insert(
            Level5,
            LevelRequirement {
                players_needed: 4,
                resources: Resources::builder()
                    .linemate(1)
                    .deraumere(2)
                    .sibur(1)
                    .mendiane(3)
                    .build(),
            },
        );
        requirements.insert(
            Level6,
            LevelRequirement {
                players_needed: 6,
                resources: Resources::builder()
                    .linemate(1)
                    .deraumere(2)
                    .sibur(3)
                    .phiras(1)
                    .build(),
            },
        );

        requirements.insert(
            Level7,
            LevelRequirement {
                players_needed: 6,
                resources: Resources::builder()
                    .linemate(2)
                    .deraumere(2)
                    .sibur(2)
                    .mendiane(2)
                    .phiras(2)
                    .thystame(1)
                    .build(),
            },
        );
        requirements
    });

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

    pub fn has_at_least(&self, required: &Resources) -> bool {
        self.contents
            .iter()
            .zip(required.contents.iter())
            .all(|(available, needed)| available >= needed)
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
        self.resources[Deraumere] = amount;
        self
    }

    pub fn linemate(mut self, amount: u64) -> Self {
        self.resources[Linemate] = amount;
        self
    }

    pub fn mendiane(mut self, amount: u64) -> Self {
        self.resources[Mendiane] = amount;
        self
    }

    pub fn phiras(mut self, amount: u64) -> Self {
        self.resources[Phiras] = amount;
        self
    }

    pub fn sibur(mut self, amount: u64) -> Self {
        self.resources[Sibur] = amount;
        self
    }

    pub fn thystame(mut self, amount: u64) -> Self {
        self.resources[Thystame] = amount;
        self
    }

    pub fn food(mut self, amount: u64) -> Self {
        self.resources[Food] = amount;
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

        assert_eq!(resources[Deraumere], 5);
        assert_eq!(resources[Linemate], 3);
        assert_eq!(resources[Food], 10);
        assert_eq!(resources[Mendiane], 0);
    }
}
