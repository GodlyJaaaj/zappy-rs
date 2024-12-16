use crate::handler::command::{CommandHandler, Handler, State};
use crate::protocol::{ClientAction, ParsingError};
use std::ops::{Deref, DerefMut};

pub struct AiHandler(Handler);

impl AiHandler {
    pub(crate) fn new(id: u64) -> Self {
        AiHandler(Handler {
            id,
            state: State::Ai,
        })
    }
}

impl Deref for AiHandler {
    type Target = Handler;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AiHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CommandHandler for AiHandler {
    fn handle_command(&mut self, team_name: String) -> Result<ClientAction, ParsingError> {
        
        Ok(ClientAction {
            client_id: self.id,
            action: crate::protocol::Action::Broadcast(team_name),
        })
    }

    fn state(&self) -> State {
        self.state
    }

    fn id(&self) -> u64 {
        self.id
    }
}