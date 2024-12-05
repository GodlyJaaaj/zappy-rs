use mio::{Events, Poll, net::TcpListener};

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
    pub fn new(addr: String,
               port: u16,
               width: u8,
               height: u8,
               teams: Vec<String>,
               clients_nb: u64,
               freq: u16) -> Self {
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
    //config: ServerConfig,
    ticks: u128,
    poll: Poll,
    events: Events,
    tcp_listener: TcpListener
}

impl Server {
    //pub fn new(config: ServerConfig) -> Self {
    //    Server {
    //        config,
    //        ticks: 0,
    //        poll: Poll::new().unwrap(),
    //        events: Events::with_capacity(2048),
    //        tcp_listener: TcpListener::bind(format!("{}:{}", config.addr, config.port).parse().unwrap()).unwrap()
    //    }
    //}

    //pub fn run(&mut self) {
    //    println!("Server is running");
    //}
}

