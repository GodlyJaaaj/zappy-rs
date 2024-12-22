use crate::protocol::ClientAction;

#[derive(Copy, Clone)]
pub enum State {
    Login,
    Ai,
    Gui,
}

pub enum HandleCommandResult {
    Ok(String),
    ChangeState(String, State),
}

pub trait CommandHandler {
    fn parse_command(&mut self, command: String) -> ClientAction;
    fn handle_command(&mut self, command: ClientAction) -> HandleCommandResult;
    fn state(&self) -> State;
    fn id(&self) -> u64;
}

pub struct Handler {
    pub(crate) id: u64,
    pub(crate) state: State,
}
