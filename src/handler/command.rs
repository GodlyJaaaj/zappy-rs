use crate::protocol::ClientAction;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Unchanged,
    Login,
    Ai,
    Gui,
}

pub trait CommandHandler {
    fn parse_command(&mut self, command: String) -> ClientAction;
    fn handle_command(&mut self, command: ClientAction, state: &mut State) -> String;
    fn state(&self) -> State;
    fn id(&self) -> u64;
}

pub struct Handler {
    pub(crate) id: u64,
    pub(crate) state: State,
}
