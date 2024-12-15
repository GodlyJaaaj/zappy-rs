use crate::protocol::ClientAction;
use tokio::sync::mpsc;

pub struct PendingClient {
    pub client_id: u64,
    pub client_tx: mpsc::Sender<ClientAction>,
}
