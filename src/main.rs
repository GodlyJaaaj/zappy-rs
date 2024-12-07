mod cell;
mod client;
mod egg;
mod gui;
mod map;
mod player;
mod resources;
mod server;
mod vec2;

use crate::server::{Server, ServerConfig};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let server_config = ServerConfig::new(
        "127.0.0.1".to_string(),
        4242,
        10,
        10,
        vec!["Team1".to_string(), "Team2".to_string()],
        4,
        1,
    );
    let mut server = Server::from_config(server_config).unwrap();
    if let Err(e) = server.try_make_readable() {
        return Err(Box::new(e));
    }
    Ok(())
}
