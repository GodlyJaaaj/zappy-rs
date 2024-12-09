use tokio::net::TcpStream;

pub struct Client {
    pub socket: TcpStream,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Client { socket }
    }
}

pub trait CommandReader {}
