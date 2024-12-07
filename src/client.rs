use mio::net::TcpStream;

pub struct Client {
    socket: TcpStream,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Client { socket }
    }
}

pub trait CommandReader {}
