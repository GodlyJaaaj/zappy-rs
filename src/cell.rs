use crate::egg::Egg;
use crate::resources::Resources;
use std::fmt;

#[derive(Clone)]
pub struct Cell {
    resources: Resources,
    eggs: Vec<Egg>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            resources: Resources::default(),
            eggs: Vec::new(),
        }
    }
}

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{GREEN}{}{RESET})", self.resources, self.eggs.len())
    }
}
