pub trait CommandHandler: Send {
    fn handle_command(&mut self, command: String);
}

