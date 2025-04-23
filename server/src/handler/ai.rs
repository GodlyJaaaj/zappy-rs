use crate::event::Event::*;
use crate::formater::{InventoryFormat, LevelFormat, LookFormat};
use crate::handler::command::State::DEAD;
use crate::handler::command::{CommandHandler, CommandRes, Handler};
use crate::protocol::{
    AIAction, AIEvent, AIResponse, EventType, HasId, Id, ServerResponse, SharedAction,
    SharedResponse,
};
use crate::resources::Resource;

pub struct AiHandler(Handler);

impl AiHandler {
    pub(crate) fn new(id: u64) -> Self {
        AiHandler(Handler { id })
    }
}

fn parse_resource(resource_name: &str) -> Option<Resource> {
    match resource_name {
        "food" => Some(Resource::Food),
        "linemate" => Some(Resource::Linemate),
        "deraumere" => Some(Resource::Deraumere),
        "sibur" => Some(Resource::Sibur),
        "mendiane" => Some(Resource::Mendiane),
        "phiras" => Some(Resource::Phiras),
        "thystame" => Some(Resource::Thystame),
        _ => None,
    }
}

impl HasId for AiHandler {
    fn id(&self) -> Id {
        self.0.id
    }
}

impl CommandHandler for AiHandler {
    fn validate_cmd(&self, cmd_name: &str, args: &str) -> EventType {
        let action = match (cmd_name, args.is_empty()) {
            // Commandes sans arguments
            ("Forward", true) => AIAction::Action(Forward),
            ("Right", true) => AIAction::Action(Right),
            ("Left", true) => AIAction::Action(Left),
            ("Look", true) => AIAction::Action(Look),
            ("Inventory", true) => AIAction::Action(Inventory),
            ("Connect_nbr", true) => AIAction::Action(ConnectNbr),
            ("Fork", true) => AIAction::Action(Fork),
            ("Eject", true) => AIAction::Action(Eject),
            ("Incantation", true) => AIAction::Action(Incantation),

            // Commandes avec arguments
            ("Broadcast", false) => AIAction::Action(Broadcast(args.to_string())),
            ("Take", false) => parse_resource(&args.to_lowercase())
                .map_or(AIAction::Shared(SharedAction::InvalidAction), |res| {
                    AIAction::Action(Take(res))
                }),
            ("Set", false) => parse_resource(&args.to_lowercase())
                .map_or(AIAction::Shared(SharedAction::InvalidAction), |res| {
                    AIAction::Action(Set(res))
                }),

            // Cas par défaut
            _ => AIAction::Shared(SharedAction::InvalidAction),
        };

        EventType::AI(AIEvent {
            id: self.id(),
            action,
        })
    }

    fn handle_command(&mut self, command: ServerResponse) -> CommandRes {
        match command {
            ServerResponse::AI(response) => match response {
                AIResponse::Shared(shared_response) => match shared_response {
                    SharedResponse::Ko => CommandRes::Response("ko\n".to_string()),
                    SharedResponse::Ok => CommandRes::Response("ok\n".to_string()),
                },
                AIResponse::Dead => CommandRes::ChangeState(DEAD("dead\n".to_string())),
                AIResponse::Broadcast(dir, str) => {
                    CommandRes::Response(format!("message {}, {}\n", dir, str))
                }
                AIResponse::Inventory(resources) => {
                    CommandRes::Response(format!("{}\n", InventoryFormat(&resources)))
                }
                AIResponse::ConnectNbr(nbr) => CommandRes::Response(format!("{}\n", nbr)),
                AIResponse::Eject(dir) => CommandRes::Response(format!("eject {}\n", dir)),
                AIResponse::Incantating => CommandRes::Response("Elevation underway\n".to_string()),
                AIResponse::LevelUp(new_level) => {
                    CommandRes::Response(format!("Current level: {}\n", LevelFormat(&new_level)))
                }
                AIResponse::Look(look_result) => {
                    CommandRes::Response(format!("{}\n", LookFormat(&look_result)))
                }
            },
            ServerResponse::Gui(_) | ServerResponse::Pending(_) => {
                unreachable!()
            }
        }
    }

    fn create_shared_event(&self, action: SharedAction) -> EventType {
        EventType::AI(AIEvent {
            id: self.id(),
            action: AIAction::Shared(action),
        })
    }
}
