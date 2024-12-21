pub struct Team {
    id: u64,
    name: String,
}

impl Team {
    pub fn new(id: u64, name: String) -> Self {
        Team { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
