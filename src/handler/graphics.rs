use crate::formater::{BctFormat, IdFormat, PinFormat, ResourceFormat};
use crate::formater::{LevelFormat, UVecFormat};
use crate::handler::command::{CommandHandler, CommandRes, Handler};
use crate::protocol::{
    EventType, GUIAction, GUIEvent, GUIResponse, HasId, Id, ServerResponse, SharedAction,
    SharedResponse, parse_prefixed_id,
};
use crate::vec2::UPosition;

pub struct GraphicHandler(Handler);

impl GraphicHandler {
    pub(crate) fn new(id: u64) -> Self {
        GraphicHandler(Handler { id })
    }
}

impl HasId for GraphicHandler {
    fn id(&self) -> Id {
        self.0.id
    }
}

impl CommandHandler for GraphicHandler {
    fn validate_cmd(&self, cmd_name: &str, args: &str) -> EventType {
        let action = match cmd_name {
            "msz" => {
                if args.is_empty() {
                    GUIAction::Msz
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "bct" => {
                let parts: Vec<&str> = args.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                        GUIAction::Bct(UPosition::new(x, y))
                    } else {
                        GUIAction::Shared(SharedAction::InvalidParameters)
                    }
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "mct" => {
                if args.is_empty() {
                    GUIAction::Mct
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "tna" => {
                if args.is_empty() {
                    GUIAction::Tna
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "ppo" => {
                if let Some(id) = parse_prefixed_id(args, '#') {
                    GUIAction::Ppo(id)
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "plv" => {
                if let Some(id) = parse_prefixed_id(args, '#') {
                    GUIAction::Plv(id)
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "pin" => {
                if let Some(id) = parse_prefixed_id(args, '#') {
                    GUIAction::Pin(id)
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "sgt" => {
                if args.is_empty() {
                    GUIAction::Sgt
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            "sst" => {
                if let Ok(t) = args.trim().parse::<u64>() {
                    GUIAction::Sst(t)
                } else {
                    GUIAction::Shared(SharedAction::InvalidParameters)
                }
            }
            &_ => GUIAction::Shared(SharedAction::InvalidAction),
        };

        EventType::GUI(GUIEvent {
            id: self.id(),
            action,
        })
    }

    fn handle_command(&mut self, command: ServerResponse) -> CommandRes {
        match command {
            ServerResponse::Gui(response) => match response {
                GUIResponse::Shared(shared) => match shared {
                    SharedResponse::Ko => CommandRes::Response("suc\n".into()),
                    SharedResponse::Ok => unreachable!(),
                },
                GUIResponse::Sbp => CommandRes::Response("sbp\n".into()),
                GUIResponse::Msz(map_size) => {
                    CommandRes::Response(format!("msz {}\n", UVecFormat(&map_size)))
                }
                GUIResponse::Bct(bct) => CommandRes::Response(format!("{}\n", BctFormat(&bct))),
                GUIResponse::Mct(mct) => {
                    let formated_mct = mct
                        .iter()
                        .map(|bct| format!("{}\n", BctFormat(bct)))
                        .collect::<Vec<String>>()
                        .join("");
                    CommandRes::Response(formated_mct)
                }
                GUIResponse::Tna(team_names) => {
                    let formated_team = team_names
                        .iter()
                        .map(|name| format!("tna {}", name))
                        .collect::<Vec<_>>()
                        .join("\n");
                    CommandRes::Response(format!("{}\n", formated_team))
                }
                GUIResponse::Ppo(player_id, player_pos, player_dir) => {
                    CommandRes::Response(format!(
                        "ppo {} {} {}\n",
                        IdFormat(&player_id),
                        UVecFormat(&player_pos),
                        i8::from(player_dir)
                    ))
                }
                GUIResponse::Plv(player_id, player_level) => CommandRes::Response(format!(
                    "plv {} {}\n",
                    IdFormat(&player_id),
                    LevelFormat(&player_level)
                )),
                GUIResponse::Pin(player_id, player_pos, player_inv) => CommandRes::Response(
                    format!("{}\n", PinFormat(&(player_id, player_pos, player_inv))),
                ),
                GUIResponse::Sgt(freq) => CommandRes::Response(format!("sgt {}\n", freq)),
                GUIResponse::Sst(freq) => CommandRes::Response(format!("sst {}\n", freq)),
                GUIResponse::Pnw(player_id, player_pos, player_dir, player_level, team_name) => {
                    CommandRes::Response(format!(
                        "pnw {} {} {} {} {}\n",
                        IdFormat(&player_id),
                        UVecFormat(&player_pos),
                        i8::from(player_dir),
                        LevelFormat(&player_level),
                        team_name
                    ))
                }
                GUIResponse::Pex(player_id) => {
                    CommandRes::Response(format!("pex {}\n", IdFormat(&player_id)))
                }
                GUIResponse::Pbc(player_id, message) => {
                    CommandRes::Response(format!("pbc {} {}\n", IdFormat(&player_id), message))
                }
                GUIResponse::Pic(pos, level, players) => {
                    let players_formatted = players
                        .iter()
                        .map(|id| format!("{}", IdFormat(id)))
                        .collect::<Vec<_>>()
                        .join(" ");

                    CommandRes::Response(format!(
                        "pic {} {} {}\n",
                        UVecFormat(&pos),
                        LevelFormat(&level),
                        players_formatted
                    ))
                }
                GUIResponse::Pie(incantation_pos, res) => CommandRes::Response(format!(
                    "pie {} {}\n",
                    UVecFormat(&incantation_pos),
                    res as u8
                )),
                GUIResponse::Pfk(player_id) => {
                    CommandRes::Response(format!("pfk {}\n", IdFormat(&player_id)))
                }
                GUIResponse::Pdr(player_id, resource) => CommandRes::Response(format!(
                    "pdr {} {}\n",
                    IdFormat(&player_id),
                    ResourceFormat(&resource)
                )),
                GUIResponse::Pgt(player_id, resource) => CommandRes::Response(format!(
                    "pgt {} {}\n",
                    IdFormat(&player_id),
                    ResourceFormat(&resource)
                )),
                GUIResponse::Pdi(player_id) => {
                    CommandRes::Response(format!("pdi {}\n", IdFormat(&player_id)))
                }
                GUIResponse::Enw(egg_id, player_id, egg_pos) => CommandRes::Response(format!(
                    "enw {} {} {}\n",
                    IdFormat(&egg_id),
                    IdFormat(&player_id),
                    UVecFormat(&egg_pos)
                )),
                GUIResponse::Ebo(egg_id) => {
                    CommandRes::Response(format!("ebo {}\n", IdFormat(&egg_id)))
                }
                GUIResponse::Edi(egg_id) => {
                    CommandRes::Response(format!("edi {}\n", IdFormat(&egg_id)))
                }
                GUIResponse::Seg(team_name) => CommandRes::Response(format!("seg {}\n", team_name)),
                GUIResponse::Smg(message) => CommandRes::Response(format!("smg {}\n", message)),
            },
            ServerResponse::AI(_) | ServerResponse::Pending(_) => {
                unreachable!()
            }
        }
    }

    fn create_shared_event(&self, action: SharedAction) -> EventType {
        EventType::GUI(GUIEvent {
            id: self.id(),
            action: GUIAction::Shared(action),
        })
    }
}
