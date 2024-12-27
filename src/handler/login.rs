use crate::handler::command::{CommandHandler, Handler, State};
use crate::protocol::Action::Login;
use crate::protocol::{Action, ClientAction, ClientType};
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
    fn parse_command(&mut self, team_name: String) -> ClientAction {
        ClientAction {
            client_id: self.id(),
            action: Login(team_name),
        }
    }

    fn handle_command(&mut self, command: ClientAction, state: &mut State) -> String {
        match command.action {
            Action::LoggedIn(client_type, nb_clients, map_size) => {
                match client_type {
                    ClientType::GUI => {
                        println!("Logged in as GUI");
			*state = State::Gui;
                        todo!("Implement GUI state");
                    }
                    ClientType::AI => {
                        println!("Logged in as AI");
			*state = State::Ai;
                        format!("{}\n{} {}\n", nb_clients, map_size.x(), map_size.y())
                    }
                }
            }
            Action::Ko => "ko\n".to_string(),
            _ => {
                println!("Unexpected action: {:?}", command.action);
                "ko\n".to_string()
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
