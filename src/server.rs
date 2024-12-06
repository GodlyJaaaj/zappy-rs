use mio::{net::TcpListener, Events, Poll, Token};
const SERVER: Token = Token(0);

pub struct ServerConfig {
    addr: String,
    port: u16,
    width: u8,
    height: u8,
    teams: Vec<String>,
    clients_nb: u64,
    freq: u16,
}

impl ServerConfig {
    pub fn new(
        addr: String,
        port: u16,
        width: u8,
        height: u8,
        teams: Vec<String>,
        clients_nb: u64,
        freq: u16,
    ) -> Self {
        ServerConfig {
            addr,
            port,
            width,
            height,
            teams,
            clients_nb,
            freq,
        }
    }
}

pub struct Server {
    ticks: u128,
    poll: Poll,
    events: Events,
    tcp_listener: TcpListener,
    //freq: u16,
    //teams
}

pub enum ServerError {
    PollError,
    FailedToParseAddr,
    FailedToBind,
    FailedToMakeReadable,
    FailedToMakeUnreadable,
}

impl Server {
    pub fn with_config(config: ServerConfig) -> Result<Self, ServerError> {
        let server = Server {
            ticks: 0,
            poll: Poll::new().map_err(|_| ServerError::PollError)?,
            events: Events::with_capacity(2048),
            tcp_listener: TcpListener::bind(
                format!("{}:{}", config.addr, config.port)
                    .parse()
                    .map_err(|_| ServerError::FailedToParseAddr)?,
            )
            .map_err(|_| ServerError::FailedToBind)?,
        };

        Ok(server)
    }

    /// Try to make the server readable
    /// This will allow the server to accept new connections
    pub fn try_make_readable(&mut self) -> Result<(), ServerError> {
        self.poll
            .registry()
            .register(&mut self.tcp_listener, SERVER, mio::Interest::READABLE)
            .map_err(|_| ServerError::FailedToMakeReadable)
    }

    /// Try to make the server unreadable
    /// This will prevent the server from accepting new connections
    pub fn try_make_unreadable(&mut self) -> Result<(), ServerError> {
        self.poll
            .registry()
            .deregister(&mut self.tcp_listener)
            .map_err(|_| ServerError::FailedToMakeUnreadable)
    }
}
