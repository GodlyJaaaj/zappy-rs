use crate::protocol::ClientAction;

#[derive(Copy, Clone)]
pub enum State {
    Login,
    Ai,
    Gui,
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: String) -> ClientAction;
    fn state(&self) -> State;
    fn id(&self) -> u64;
}

pub struct Handler {
    pub(crate) id: u64,
    pub(crate) state: State,
}
