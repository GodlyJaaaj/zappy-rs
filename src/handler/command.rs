use crate::protocol::{EventType, HasId, ServerResponse, SharedAction};

pub enum State {
    IA(String),
    GUI,
    DEAD(String),
}

pub enum CommandRes {
    ChangeState(State),
    Response(String),
}

pub trait CommandHandler: HasId {
    fn validate_cmd(&self, cmd_name: &str, args: &str) -> EventType;
    fn parse_command(&mut self, command: String) -> EventType {
        let split_cmd = split_command(&command);
        let cmd_name = split_cmd.0;
        let args = split_cmd.1;

        self.validate_cmd(cmd_name, args)
    }
    fn handle_command(&mut self, command: ServerResponse) -> CommandRes;
    fn create_shared_event(&self, action: SharedAction) -> EventType;
}

pub struct Handler {
    pub(crate) id: u64,
}

pub fn split_command(full_cmd: &str) -> (&str, &str) {
    match full_cmd.split_once(' ') {
        Some((cmd_name, args)) => (cmd_name, args),
        None => (full_cmd, ""),
    }
}
