#![allow(dead_code)]
#![allow(unused_variables)]

mod cell;
mod connection;
mod egg;
mod gui;
mod handler;
mod map;
mod player;
mod protocol;
mod resources;
mod server;
mod vec2;
mod team;

use crate::server::{Server, ServerConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
