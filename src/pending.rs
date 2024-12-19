use crate::protocol::{ClientAction, Ko};
use tokio::sync::mpsc::Sender;

pub struct PendingClient {
    pub client_id: u64,
    pub client_tx: Sender<ClientAction>,
}

impl PendingClient {
    pub(crate) fn client_tx(&self) -> Sender<ClientAction> {
        self.client_tx.clone()
    }
}

impl PendingClient {
    pub fn id(&self) -> u64 {
        self.client_id
    }
}

impl Ko for PendingClient {
    async fn ko(&mut self) -> bool {
        self.client_tx
            .send(ClientAction {
                client_id: self.id(),
                action: crate::protocol::Action::Ko,
            })
            .await
            .is_ok()
    }
}
