mod cell;
mod client;
mod egg;
mod gui;
mod map;
mod player;
mod resources;
mod server;
mod vec2;

use crate::map::Map;
use crate::server::{Server, ServerConfig};
use crate::vec2::Size;
use mio::Token;
use std::error::Error;
use std::io::Read;

const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    let map = Map::new(Size::from((5, 5)));
    println!("{}", map);
    //WIP
    let server_config = ServerConfig::new(
        "127.0.0.1".to_string(),
        4242,
        10,
        10,
        vec!["Team1".to_string(), "Team2".to_string()],
        4,
        1,
    );
    //let mut server = Server::new(server_config);
    Ok(())
}
