use crate::event::Event::*;
use crate::handler::command::{CommandHandler, CommandRes, Handler};
use crate::protocol::{AIAction, AIEvent, EventType, HasId, Id, ServerResponse, SharedAction};
use crate::resources::Resource;

pub struct AiHandler(Handler);

impl AiHandler {
    pub(crate) fn new(id: u64) -> Self {
        AiHandler(Handler { id })
    }

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

fn split_command(full_cmd: &str) -> (&str, &str) {
    match full_cmd.split_once(' ') {
        Some((cmd_name, args)) => (cmd_name, args),
        None => (full_cmd, ""),
    }
}

impl HasId for AiHandler {
    fn id(&self) -> Id {
        self.0.id
    }
}

impl CommandHandler for AiHandler {
    fn parse_command(&mut self, full_cmd: String) -> EventType {
        let split_cmd = split_command(&full_cmd);
        let cmd_name = split_cmd.0;
        let args = split_cmd.1;

        self.validate_cmd(cmd_name, args)
    }

    fn handle_command(&mut self, command: ServerResponse) -> CommandRes {
        CommandRes::Response("".to_string())
    }

    fn create_shared_event(&self, action: SharedAction) -> EventType {
        EventType::AI(AIEvent {
            id: self.id(),
            action: AIAction::Shared(action),
        })
    }
}

//match command.action {
//             Action::Ok => "ok\n".to_string(),
//             Action::Ko => "ko\n".to_string(),
//             Action::Broadcast(dir, message) => {
//                 if self.id() == command.client_id {
//                     "ok\n".to_string()
//                 } else {
//                     format!("message {}, {}\n", dir, message)
//                 }
//             }
//             Action::Look => {
//                 todo!("Implement look")
//             }
//             Action::Inventory(inv) => {
//                 format!(
//                     "[deraumere {}, linemate {}, mendiane {}, phiras {}, sibur {}, thystame {}, food {}]\n",
//                     inv[Resource::Deraumere],
//                     inv[Resource::Linemate],
//                     inv[Resource::Mendiane],
//                     inv[Resource::Phiras],
//                     inv[Resource::Sibur],
//                     inv[Resource::Thystame],
//                     inv[Resource::Food]
//                 )
//             }
//             Action::ConnectNbr => {
//                 todo!("Implement connect_nbr")
//             }
//             Action::Fork => {
//                 todo!("Implement fork")
//             }
//             Action::Eject => {
//                 todo!("Implement eject")
//             }
//             Action::Take(_) => {
//                 todo!("Implement take")
//             }
//             Action::Set(_) => {
//                 todo!("Implement set")
//             }
//             Action::Incantation => {
//                 todo!("Implement incantation")
//             }
//             _ => {
//                 todo!("should not be there")
//             }
//         }
