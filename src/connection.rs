use crate::handler::command::CommandHandler;
use crate::handler::login::LoginHandler;
use crate::protocol::{Action, ClientAction};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct Connection {
    id: u64,
    stream: BufReader<TcpStream>,
    command_handler: Box<dyn CommandHandler + Send>,
}

#[derive(Debug)]
enum ConnectionError {
    Closed,
    IO(tokio::io::Error),
}

impl Connection {
    pub fn new(id: u64, socket: TcpStream) -> Self {
        Connection {
            id,
            stream: BufReader::new(socket),
            command_handler: Box::new(LoginHandler::new(id)),
        }
    }

    pub async fn handle(
        &mut self,
        tx: mpsc::Sender<ClientAction>,
        mut rx: mpsc::Receiver<ClientAction>,
    ) {
        loop {
            if let Err(e) = self.update(&tx, &mut rx).await {
                println!("End of connection: {:?}", e);
                tx.send(ClientAction {
                    client_id: self.id,
                    action: Action::Disconnect,
                })
                .await
                .expect("Failed to send disconnect");
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

    async fn update(
        &mut self,
        tx: &mpsc::Sender<ClientAction>,
        rx: &mut mpsc::Receiver<ClientAction>,
    ) -> Result<(), ConnectionError> {
        tokio::select! {
            cmd = self.read_line() => {
                let line = cmd?;
                let action = self.command_handler.handle_command(line).unwrap();
                tx.send(action).await.expect("Could not send action");
                Ok(())
            }
            res = rx.recv() => {
                let res = res.expect("Could not receive action from channel");
                Ok(())
            }
        }
    }
}
