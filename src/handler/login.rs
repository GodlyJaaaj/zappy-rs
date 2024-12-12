use crate::handler::command::CommandHandler;

pub struct LoginHandler{}

impl LoginHandler {
    pub(crate) fn new() -> Self {
        LoginHandler{}
    }
}

impl CommandHandler for LoginHandler {
    fn handle_command(&mut self, command: String) {
        println!("Not logged in yet");
    }
}