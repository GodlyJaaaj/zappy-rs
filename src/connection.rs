use crate::handler::command::{CommandHandler, State};
use crate::handler::login::LoginHandler;
use crate::protocol::{Action, ClientAction, ClientType};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use crate::handler::ai::AiHandler;

pub struct Connection {
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
                    client_id: self.command_handler.id(),
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

    async fn login_state(&mut self, res: ClientAction) {
        match res.action {
            Action::LoggedIn(t, nb_clients, map_size) => {
                match t {
                    //if the client logged in as GUI
                    ClientType::GUI => {
                        println!("Logged in as GUI");
                        todo!("Implement GUI state");
                    }
                    //if the client logged in as AI
                    ClientType::AI => {
                        println!("Logged in as AI");
                        self.command_handler = Box::new(AiHandler::new(self.command_handler.id()));
                    }
                }
            }
            Action::Ko => {
                println!("Login failed");
                let _ = self.stream
                    .write_all(b"ko\n").await;
            }
            _ => {
                println!("Unexpected action: {:?}", res.action);
            }
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
                match self.command_handler.state() {
                    State::Login => {
                        self.login_state(res).await;
                    }
                    State::Ai => {
                        todo!("Implement Ai state in connection::update");
                    }
                    State::Gui => {
                        todo!("Implement Gui state");
                    }
                }
                Ok(())
            }
        }
    }
}
