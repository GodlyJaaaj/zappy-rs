use crate::protocol::{EventType, HasId, ServerResponse, SharedAction};

pub enum State {
    IA(String),
    GUI(String),
    DEAD(String),
}

pub enum CommandRes {
    ChangeState(State),
    Response(String),
}

pub trait CommandHandler: HasId {
    fn parse_command(&mut self, command: String) -> EventType;
    fn handle_command(&mut self, command: ServerResponse) -> CommandRes;
    fn create_shared_event(&self, action: SharedAction) -> EventType;
}

pub struct Handler {
    pub(crate) id: u64,
}
