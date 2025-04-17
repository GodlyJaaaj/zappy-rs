use crate::pending::PendingClient;
use crate::protocol::{ClientSender, HasId, Id, ServerResponse};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Gui {
    id: Id,
    gui_tx: Sender<ServerResponse>,
}

impl HasId for Gui {
    fn id(&self) -> Id {
        self.id
    }
}

impl ClientSender for Gui {
    fn get_client_tx(&self) -> &Sender<ServerResponse> {
        &self.gui_tx
    }
}

pub struct GuiBuilder {
    id: Option<Id>,
    gui_tx: Option<Sender<ServerResponse>>,
}

impl GuiBuilder {
    pub fn new() -> Self {
        GuiBuilder {
            id: None,
            gui_tx: None,
        }
    }

    pub fn id(self, id: Id) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn pending_client(mut self, pending_client: PendingClient) -> Self {
        self.id = Some(pending_client.id());
        self.gui_tx = Some(pending_client.client_tx);
        self
    }

    pub fn build(self) -> Result<Gui, &'static str> {
        let gui_tx = self.gui_tx.ok_or("GUI channel is required")?;
        let id = self.id.ok_or("GUI ID is required")?;

        Ok(Gui { id, gui_tx })
    }
}
