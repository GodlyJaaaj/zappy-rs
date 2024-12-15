use crate::protocol::{ClientAction, ParsingError};

pub trait CommandHandler {
    fn handle_command(&mut self, command: String) -> Result<ClientAction, ParsingError>;
}
