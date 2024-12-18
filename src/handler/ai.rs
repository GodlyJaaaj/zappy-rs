use crate::handler::command::{CommandHandler, Handler, State};
use crate::protocol::{Action, ClientAction, ParsingError};
use std::ops::{Deref, DerefMut};
use crate::resources::Resource;

pub struct AiHandler(Handler);

impl AiHandler {
    pub(crate) fn new(id: u64) -> Self {
        AiHandler(Handler {
            id,
            state: State::Ai,
        })
    }

    fn validate_cmd(&self,  cmd_name: &str, args: &str) -> Action {
        match cmd_name {
            "Forward" if args.is_empty() => {
                    Action::Forward
            }
            "Broadcast" if !args.is_empty() => {
                    Action::Broadcast(args.into())
            }
            "Right" if !args.is_empty() => {
                    Action::Right
            }
            "Left" if !args.is_empty() => {
                    Action::Left
            }
            "Look" if args.is_empty() => {
                    Action::Look
            }
            "Inventory" if args.is_empty() => {
                    Action::Inventory
            }
            "Connect_nbr" if args.is_empty() => {
                    Action::ConnectNbr
            }
            "Fork" if args.is_empty() => {
                    Action::Fork
            }
            "Eject" if args.is_empty() => {
                Action::Eject
            }
            "Take" if !args.is_empty() => {
                    //todo: Parse resource arg
                    Action::Take(Resource::Food)
            }
            "Set" if !args.is_empty() => {
                    //todo: Parse resource arg
                    Action::Set(Resource::Food)
            }
            "Incantation" if args.is_empty() => {
                    Action::Incantation
            }
            &_ => {
                Action::Ko
            }
        }
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

fn split_command(full_cmd: &str) -> (&str, &str) {
    match full_cmd.split_once(' ') {
        Some((cmd_name, args)) => (cmd_name, args),
        None => (full_cmd, ""),
    }
}

impl CommandHandler for AiHandler {
    fn handle_command(&mut self, full_cmd: String) -> ClientAction {
        let split_cmd = split_command(&full_cmd);
        let cmd_name = split_cmd.0;
        let args = split_cmd.1;

        let parse_res = self.validate_cmd(cmd_name, args);
        ClientAction {
            client_id: self.id(),
            action: parse_res,
        }
    }

    fn state(&self) -> State {
        self.state
    }

    fn id(&self) -> u64 {
        self.id
    }
}