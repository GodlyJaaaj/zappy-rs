use crate::handler::command::{CommandHandler, Handler, State};
use crate::protocol::Action::Login;
use crate::protocol::{ClientAction, ParsingError};
use std::ops::{Deref, DerefMut};

pub struct LoginHandler(Handler);

impl LoginHandler {
    pub(crate) fn new(id: u64) -> Self {
        LoginHandler(Handler {
            id,
            state: State::Login,
        })
    }
}

impl Deref for LoginHandler {
    type Target = Handler;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LoginHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CommandHandler for LoginHandler {
    fn handle_command(&mut self, mut team_name: String) -> Result<ClientAction, ParsingError> {
        team_name.pop(); // remove the newline char
        let action = ClientAction {
            client_id: self.id(),
            action: Login(team_name),
        };
        Ok(action)
    }

    fn state(&self) -> State {
        self.state
    }

    fn id(&self) -> u64 {
        self.id
    }
}