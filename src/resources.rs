use std::fmt;

#[derive(Default, Clone, Debug)]
pub struct Resources {
    deraumere: u64,
    linemate: u64,
    mendiane: u64,
    phiras: u64,
    sibur: u64,
    thystame: u64,
    food: u64,
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{},{},{},{}", self.deraumere, self.linemate, self.mendiane, self.phiras, self.sibur, self.thystame, self.food)
    }
}