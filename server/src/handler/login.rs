use crate::handler::command::CommandRes::ChangeState;
use crate::handler::command::State::GUI;
use crate::handler::command::{CommandHandler, CommandRes, Handler, State};
use crate::protocol::{
    EventType, HasId, Id, PendingAction, PendingEvent, PendingResponse, ServerResponse,
    SharedAction, SharedResponse, TeamType,
};
use log::warn;

pub struct LoginHandler(Handler);

impl LoginHandler {
    pub(crate) fn new(id: Id) -> Self {
        LoginHandler(Handler { id })
    }
}

impl HasId for LoginHandler {
    fn id(&self) -> Id {
        self.0.id
    }
}

impl CommandHandler for LoginHandler {
    fn validate_cmd(&self, _: &str, _: &str) -> EventType {
        unreachable!()
    }

    fn parse_command(&mut self, team_name: String) -> EventType {
        EventType::Pending(PendingEvent {
            id: self.id(),
            action: PendingAction::Login(team_name),
        })
    }

    fn handle_command(&mut self, command: ServerResponse) -> CommandRes {
        match command {
            ServerResponse::Pending(response) => match response {
                PendingResponse::Shared(shared) => match shared {
                    SharedResponse::Ko => CommandRes::Response("ko\n".to_string()),
                    SharedResponse::Ok => CommandRes::Response("ok\n".to_string()),
                },
                PendingResponse::LogAs(team) => match team {
                    TeamType::Graphic => ChangeState(GUI),
                    TeamType::IA(client_num, map_size) => ChangeState(State::IA(format!(
                        "{}\n{} {}\n",
                        client_num,
                        map_size.x(),
                        map_size.y()
                    ))),
                },
            },
            _ => {
                warn!("Received invalid command: {:?}", command);
                CommandRes::Response("ko\n".to_owned())
            }
        }
    }

    fn create_shared_event(&self, action: SharedAction) -> EventType {
        EventType::Pending(PendingEvent {
            id: self.id(),
            action: PendingAction::Shared(action),
        })
    }
}
