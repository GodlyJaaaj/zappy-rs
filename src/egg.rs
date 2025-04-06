use crate::protocol::{HasId, Id};
use crate::vec2::{HasPosition, Position};

#[derive(Clone, Debug)]
pub struct Egg {
    team_id: Id,
    pos: Position,
}

impl Egg {
    pub fn new(team_id: Id, pos: Position) -> Self {
        Egg { team_id, pos }
    }
}

impl HasId for Egg {
    fn id(&self) -> Id {
        self.team_id
    }
}

impl HasPosition for Egg {
    fn position(&self) -> Position {
        self.pos
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.pos
    }
}
