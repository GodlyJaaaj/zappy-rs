#![allow(dead_code)]
#![allow(unused_variables)]

mod cell;
mod connection;
mod egg;
mod event;
mod gui;
mod handler;
mod map;
mod pending;
mod player;
mod protocol;
mod resources;
mod server;
mod sound;
mod team;
mod vec2;

use crate::server::{Server, ServerConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let server_config = ServerConfig::new(
        "0.0.0.0".to_string(),
        4242,
        10,
        10,
        vec!["Team1".to_string(), "Team2".to_string()],
        4,
        1,
    );
    let mut server = Server::from_config(server_config).await?;
    server.run().await?;
    Ok(())
}
