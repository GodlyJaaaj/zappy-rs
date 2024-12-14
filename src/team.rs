use crate::player::Player;

pub struct Team {
    id: usize,
    name: String,
    players: Vec<Player>,
    max_clients: u64,
}

impl Team {
    pub fn new(id: usize, name: String, max_clients: u64) -> Self {
        Team {
            id,
            name,
            players: Vec::new(),
            max_clients
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }
}