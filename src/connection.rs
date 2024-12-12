use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufReader<TcpStream>,
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
                println!("{}", line);
                Ok(())
            }
        }
    }
}
