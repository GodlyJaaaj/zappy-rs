use crate::handler::command::{CommandHandler, HandleCommandResult, Handler, State};
use crate::protocol::{Action, ClientAction};
use crate::resources::Resource;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct AiHandler(Handler);

impl AiHandler {
    pub(crate) fn new(id: u64) -> Self {
        AiHandler(Handler {
            id,
            state: State::Ai,
        })
    }

    fn validate_cmd(&self, cmd_name: &str, args: &str) -> Action {
        if args.is_empty() {
            match cmd_name {
                "Forward" => Action::Forward,
                "Right" => Action::Right,
                "Left" => Action::Left,
                "Look" => Action::Look,
                "Inventory" => Action::Inventory,
                "Connect_nbr" => Action::ConnectNbr,
                "Fork" => Action::Fork,
                "Eject" => Action::Eject,
                "Incantation" => Action::Incantation,
                &_ => Action::Ko,
            }
        } else {
            match cmd_name {
                "Broadcast" => Action::Broadcast(0, Arc::new(args.into())),
                "Take" => Action::Take(Resource::Food),
                "Set" => Action::Set(Resource::Food),
                &_ => Action::Ko,
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
    fn parse_command(&mut self, full_cmd: String) -> ClientAction {
        let split_cmd = split_command(&full_cmd);
        let cmd_name = split_cmd.0;
        let args = split_cmd.1;

        let parse_res = self.validate_cmd(cmd_name, args);
        ClientAction {
            client_id: self.id(),
            action: parse_res,
        }
    }

    fn handle_command(&mut self, command: ClientAction) -> HandleCommandResult {
        match command.action {
            Action::Ok => HandleCommandResult::Ok("ok\n".to_string()),
            Action::Ko => HandleCommandResult::Ok("ko\n".to_string()),
            Action::Broadcast(dir, message) => {
                if self.id() == command.client_id {
                    return HandleCommandResult::Ok("ok\n".to_string());
                }
                HandleCommandResult::Ok(format!("message {}, {}\n", dir, message))
            }
            Action::Forward => {
                todo!("Implement forward")
            }
            Action::Right => {
                todo!("Implement right")
            }
            Action::Left => {
                todo!("Implement left")
            }
            Action::Look => {
                todo!("Implement look")
            }
            Action::Inventory => {
                todo!("Implement inventory")
            }
            Action::ConnectNbr => {
                todo!("Implement connect_nbr")
            }
            Action::Fork => {
                todo!("Implement fork")
            }
            Action::Eject => {
                todo!("Implement eject")
            }
            Action::Take(_) => {
                todo!("Implement take")
            }
            Action::Set(_) => {
                todo!("Implement set")
            }
            Action::Incantation => {
                todo!("Implement incantation")
            }
            _ => {
                todo!("should not be there")
            }
        }
    }

    fn state(&self) -> State {
        self.state
    }

    fn id(&self) -> u64 {
        self.id
    }
}
