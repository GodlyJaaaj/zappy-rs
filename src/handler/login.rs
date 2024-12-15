use crate::handler::command::CommandHandler;
use crate::protocol::Action::Login;
use crate::protocol::{ClientAction, ParsingError};

pub struct LoginHandler {
    id: u64,
}

impl LoginHandler {
    pub(crate) fn new(id: u64) -> Self {
        LoginHandler { id }
    }
}

impl CommandHandler for LoginHandler {
    fn handle_command(&mut self, team_name: String) -> Result<ClientAction, ParsingError> {
        let action = ClientAction {
            client_id: self.id,
            action: Login(team_name),
        };
        Ok(action)
    }
}
