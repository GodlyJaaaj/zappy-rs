use crate::protocol::{HasId, Id};
use crate::vec2::{HasPosition, UPosition};

#[derive(Clone, Debug)]
pub struct Egg {
    team_id: Id,
    pos: UPosition,
}

impl Egg {
    pub fn new(team_id: Id, pos: UPosition) -> Self {
        Egg { team_id, pos }
    }
}

impl HasId for Egg {
    fn id(&self) -> Id {
        self.team_id
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
