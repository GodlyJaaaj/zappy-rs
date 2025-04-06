use crate::protocol::{ClientSender, HasId, Id, ServerResponse};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct PendingClient {
    pub client_id: u64,
    pub client_tx: Sender<ServerResponse>,
}

impl HasId for PendingClient {
    fn id(&self) -> Id {
        self.client_id
    }
}

impl ClientSender for PendingClient {
    fn get_client_tx(&self) -> &Sender<ServerResponse> {
        &self.client_tx
    }
}
