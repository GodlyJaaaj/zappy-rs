mod cell;
mod client;
mod gui;
mod map;
mod player;
mod resources;
mod server;
mod vec2;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::time::Duration;

const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut clients: HashMap<Token, TcpStream> = HashMap::new();
    let mut next_token = Token(SERVER.0 + 1);

    let addr = "127.0.0.1:4242".parse()?;
    let mut server = TcpListener::bind(addr)?;
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, Option::from(Duration::from_secs(0)))?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let (mut connection, _address) = server.accept()?;
                    let token = next_token;
                    next_token.0 += 1;
                    poll.registry()
                        .register(&mut connection, token, Interest::READABLE)?;
                    clients.insert(token, connection);
                }
                token => {
                    if let Some(client) = clients.get_mut(&token) {
                        let mut buf = [0; 1024];
                        match client.read(&mut buf) {
                            Ok(0) => {
                                clients.remove(&token);
                            }
                            Ok(n) => {
                                println!(
                                    "Received message from {:?}: {:?}",
                                    token,
                                    String::from_utf8_lossy(&buf[..n])
                                );
                            }
                            Err(e) => {
                                eprintln!("Error reading from client: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}
