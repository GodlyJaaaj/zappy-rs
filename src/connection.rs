use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use crate::handler::command::CommandHandler;
use crate::handler::login::LoginHandler;

pub struct Connection {
    stream: BufReader<TcpStream>,
    command_handler: Box<dyn CommandHandler>
}

#[derive(Debug)]
enum ConnectionError {
    Closed,
    IO(tokio::io::Error),
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufReader::new(socket),
            command_handler: Box::new(LoginHandler::new())
        }
    }

    pub async fn handle(&mut self) {
        loop {
            if let Err(e) = self.update().await {
                println!("End of connection: {:?}", e);
                break;
            }
        }
    }

    async fn read_line(&mut self) -> Result<String, ConnectionError> {
        let mut line = String::new();
        match self.stream.read_line(&mut line).await {
            Ok(0) => Err(ConnectionError::Closed),
            Ok(_) => Ok(line),
            Err(e) => Err(ConnectionError::IO(e)),
        }
    }

    async fn update(&mut self) -> Result<(), ConnectionError> {
        tokio::select! {
            val = self.read_line() => {
                let line = val?;
                self.command_handler.handle_command(line);
                Ok(())
            }
        }
    }
}
