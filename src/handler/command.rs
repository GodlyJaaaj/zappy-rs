pub trait CommandHandler {
    fn handle_command(&mut self, command: String);
}
