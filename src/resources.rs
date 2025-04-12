use crate::resources::ElevationLevel::*;
use core::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::fmt;
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

#[repr(u8)]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum ElevationLevel {
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

pub struct LevelFormat<'a>(pub &'a ElevationLevel);

impl fmt::Display for LevelFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Level1 => 1,
                Level2 => 2,
                Level3 => 3,
                Level4 => 4,
                Level5 => 5,
                Level6 => 6,
                Level7 => 7,
                Level8 => 8,
            }
        )
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
            ElevationLevel::Level1,
            LevelRequirement {
                players_needed: 1,
                resources: Resources::builder().linemate(1).build(),
            },
        );

        requirements.insert(
            ElevationLevel::Level2,
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

pub struct InventoryFormat<'a>(pub &'a Resources);

impl fmt::Display for InventoryFormat<'_> {
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
