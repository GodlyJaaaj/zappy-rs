use crate::protocol::{HasId, Id};

pub struct Team {
    id: u64,
    name: String,
}

impl Team {
    pub fn new(id: Id, name: String) -> Self {
        Team { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl HasId for Team {
    fn id(&self) -> Id {
        self.id
    }
}
