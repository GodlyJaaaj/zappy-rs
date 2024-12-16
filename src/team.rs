use crate::player::Player;
use crate::protocol::ClientAction;
use crate::vec2::Size;

pub struct Team {
    id: usize,
    name: String,
    pub(crate) players: Vec<Player>,
    max_clients: u64,
}

impl Team {
    pub fn new(id: usize, name: String, max_clients: u64) -> Self {
        Team {
            id,
            name,
            players: Vec::new(),
            max_clients,
        }
    }

    pub async fn add_player(&mut self, player: Player, map_size: Size) {
        println!("Player {} joined team {}", player.id(), self.name);
        player
            .send(ClientAction {
                client_id: player.id(),
                action: crate::protocol::Action::LoggedIn(
                    crate::protocol::ClientType::AI,
                    self.max_clients - self.players.len() as u64,
                    (map_size.x() as u8, map_size.y() as u8),
                ),
            })
            .await;
        self.players.push(player);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
