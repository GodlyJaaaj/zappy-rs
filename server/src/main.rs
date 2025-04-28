#![allow(dead_code)]

mod cell;
mod connection;
mod constant;
mod egg;
mod event;
mod formater;
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
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let server_config = ServerConfig::new(
        "0.0.0.0".to_string(),
        4242,
        20,
        20,
        vec![
            "team1".to_string(),
            "Team1".to_string(),
            "Team2".to_string(),
            "GRAPHIC".to_string(),
        ],
        4,
        100,
    );
    let mut server = Server::from_config(server_config).await?;
    server.run().await?;
    Ok(())
}
