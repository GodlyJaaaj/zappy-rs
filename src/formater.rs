use crate::protocol::{BctResponse, Id, LookResult};
use crate::resources::ElevationLevel::{
    Level0, Level1, Level2, Level3, Level4, Level5, Level6, Level7, Level8,
};
use crate::resources::Resource::{Deraumere, Food, Linemate, Mendiane, Sibur};
use crate::resources::Resource::{Phiras, Thystame};
use crate::resources::{ElevationLevel, Resource, Resources};
use crate::vec2::UPosition;
use std::fmt;

pub struct IdFormat<'a>(pub &'a Id);

impl fmt::Display for IdFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

pub struct UVecFormat<'a>(pub &'a UPosition);

impl fmt::Display for UVecFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0.x(), self.0.y())
    }
}

pub struct LookFormat<'a>(pub &'a LookResult);

impl fmt::Display for LookFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cells = self.0;
        let mut formatted_cells = Vec::new();

        for (player_count, resources) in cells {
            let mut cell_elements = Vec::new();

            // Add players
            for _ in 0..*player_count {
                cell_elements.push("player".to_string());
            }

            // Add resources
            let resource_names = [
                ("food", Food),
                ("linemate", Linemate),
                ("deraumere", Deraumere),
                ("sibur", Sibur),
                ("mendiane", Mendiane),
                ("phiras", Phiras),
                ("thystame", Thystame),
            ];

            for &(name, index) in &resource_names {
                for _ in 0..resources[index] {
                    cell_elements.push(name.to_string());
                }
            }

            if !formatted_cells.is_empty() {
                cell_elements.insert(0, "".to_string());
            }

            formatted_cells.push(cell_elements.join(" "));
        }

        write!(f, "[{}]", formatted_cells.join(","))
    }
}

pub struct BctFormat<'a>(pub &'a BctResponse);

impl fmt::Display for BctFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bct {} {}",
            UVecFormat(&self.0.0),
            ResourcesFormat(&self.0.1),
        )
    }
}

pub struct PinFormat<'a>(pub &'a (Id, UPosition, Resources));

impl fmt::Display for PinFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pin {} {} {}",
            IdFormat(&self.0.0),
            UVecFormat(&self.0.1),
            ResourcesFormat(&self.0.2)
        )
    }
}

pub struct LevelFormat<'a>(pub &'a ElevationLevel);

impl fmt::Display for LevelFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Level0 => 0,
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

pub struct InventoryFormat<'a>(pub &'a Resources);

impl fmt::Display for InventoryFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.0;
        write!(
            f,
            "[deraumere {}, linemate {}, mendiane {}, phiras {}, sibur {}, thystame {}, food {}]",
            r[Deraumere], r[Linemate], r[Mendiane], r[Phiras], r[Sibur], r[Thystame], r[Food]
        )
    }
}

pub struct ResourcesFormat<'a>(pub &'a Resources);

impl fmt::Display for ResourcesFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.0[Food],
            self.0[Linemate],
            self.0[Deraumere],
            self.0[Sibur],
            self.0[Mendiane],
            self.0[Phiras],
            self.0[Thystame]
        )
    }
}

//only use for gui index
pub struct ResourceFormat<'a>(pub &'a Resource);
impl fmt::Display for ResourceFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Deraumere => {
                    2
                }
                Linemate => {
                    1
                }
                Mendiane => {
                    4
                }
                Phiras => {
                    5
                }
                Sibur => {
                    3
                }
                Thystame => {
                    6
                }
                Food => {
                    0
                }
            }
        )
    }
}
