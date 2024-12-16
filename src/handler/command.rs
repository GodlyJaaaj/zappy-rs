use crate::protocol::{ClientAction, ParsingError};

#[derive(Copy, Clone)]
pub enum State {
    Login,
    Ai,
    Gui,
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: String) -> Result<ClientAction, ParsingError>;
    fn state(&self) -> State;
    fn id(&self) -> u64;
}

pub struct Handler { 
    pub(crate) id: u64, 
    pub(crate) state: State,
}