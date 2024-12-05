mod cell;
mod client;
mod gui;
mod map;
mod player;
mod resources;
mod server;
mod vec2;
mod egg;

use crate::map::Map;
use crate::vec2::Size;
use mio::Token;
use std::error::Error;
use std::io::Read;

const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    let map = Map::new(Size::from((5, 5)));
    println!("{}", map);
    Ok(())
}
