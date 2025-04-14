use crate::protocol::{HasId, Id};
use crate::vec2::{HasPosition, UPosition};

#[derive(Clone, Debug)]
pub struct Egg {
    id: Id,
    team_id: Id,
    pos: UPosition,
}

impl Egg {
    pub fn new(id: Id, team_id: Id, pos: UPosition) -> Self {
        Egg { id, team_id, pos }
    }

    pub fn team_id(&self) -> Id {
        self.team_id
    }
}

impl HasId for Egg {
    fn id(&self) -> Id {
        self.id
    }
}

impl HasPosition for Egg {
    fn position(&self) -> UPosition {
        self.pos
    }

    fn position_mut(&mut self) -> &mut UPosition {
        &mut self.pos
    }
}
