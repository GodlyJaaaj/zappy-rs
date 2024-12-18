use crate::pending::PendingClient;
use crate::protocol::{ClientAction, Ko};
use crate::resources::Resources;
use crate::vec2::Position;
use tokio::sync::mpsc;
use crate::protocol;

#[derive(Clone, Debug)]
pub struct Player {
    team: String,
    id: u64,
    inventory: Resources,
    pos: Position,
    elevation: u8,
    satiety: u8,
    client_tx: mpsc::Sender<ClientAction>,
}

impl Player {
    pub fn new(team: String, pending_client: PendingClient) -> Self {
        Player {
            team,
            id: pending_client.id(),
            inventory: Resources::default(),
            pos: (0, 0).into(), // todo!
            elevation: 1,
            satiety: 10, // todo!
            client_tx: pending_client.client_tx(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub async fn send(&self, action: ClientAction) {
        let _ = self.client_tx.send(action).await;
    }
}

impl Ko for Player {
    async fn ko(&mut self) -> bool {
        self.client_tx.send(
            ClientAction {
                client_id: self.id(),
                action: protocol::Action::Ko,
            }
        ).await.is_ok()
    }
}