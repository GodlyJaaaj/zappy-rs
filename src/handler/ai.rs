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

    fn validate_cmd(&self,  cmd_name: &str, args: &str) -> Result<Action, ParsingError> {
        match cmd_name {
            "Forward" => {
                if args.is_empty() {
                    Ok(Action::Forward)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Broadcast" => {
                if args.is_empty() {
                    Err(ParsingError::InvalidAction)
                } else {
                    Ok(Action::Broadcast(args.into()))
                }
            }
            "Right" => {
                if args.is_empty() {
                    Err(ParsingError::InvalidAction)
                } else {
                    Ok(Action::Right)
                }
            }
            "Left" => {
                if args.is_empty() {
                    Err(ParsingError::InvalidAction)
                } else {
                    Ok(Action::Left)
                }
            }
            "Look" => {
                if args.is_empty() {
                    Ok(Action::Look)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Inventory" => {
                if args.is_empty() {
                    Ok(Action::Inventory)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Connect_nbr" => {
                if args.is_empty() {
                    Ok(Action::ConnectNbr)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Fork" => {
                if args.is_empty() {
                    Ok(Action::Fork)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Eject" => {
                if args.is_empty() {
                    Ok(Action::Eject)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            "Take" => {
                if args.is_empty() {
                    Err(ParsingError::InvalidAction)
                } else {
                    //todo: Parse resource arg
                    Ok(Action::Take(Resource::Food))
                }
            }
            "Set" => {
                if args.is_empty() {
                    Err(ParsingError::InvalidAction)
                } else {
                    //todo: Parse resource arg
                    Ok(Action::Set(Resource::Food))
                }
            }
            "Incantation" => {
                if args.is_empty() {
                    Ok(Action::Incantation)
                } else {
                    Err(ParsingError::InvalidAction)
                }
            }
            &_ => {
                Err(ParsingError::InvalidAction)
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

impl CommandHandler for AiHandler {
    fn handle_command(&mut self, full_cmd: String) -> Result<ClientAction, ParsingError> {
        let split_cmd = full_cmd.split_once(' ');
        if split_cmd.is_none() {
            return Ok(ClientAction {
                client_id: self.id(),
                action: Action::Ko
            });
        }
        let cmd_name = split_cmd.unwrap().0;
        let args = split_cmd.unwrap().1;

        let parse_res = self.validate_cmd(cmd_name, args);
        if let Ok(action) = parse_res {
            Ok(ClientAction {
                client_id: self.id(),
                action
            })
        } else {
            Err(ParsingError::InvalidAction)
        }
    }

    fn state(&self) -> State {
        self.state
    }

    fn id(&self) -> u64 {
        self.id
    }
}