use crate::handler::ai::AiHandler;
use crate::handler::command::{CommandHandler, HandleCommandResult, State};
use crate::handler::login::LoginHandler;
use crate::protocol::{Action, ClientAction, Ko};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::io::{AsyncReadExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct Connection {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
    command_handler: Box<dyn CommandHandler + Send>,
}

#[derive(Debug)]
enum ConnectionError {
    ReachedTakeLimit,
    Closed,
    IO(tokio::io::Error),
}

impl Connection {
    pub fn new(id: u64, socket: TcpStream) -> Self {
        let (read_half, write_half) = socket.into_split();
        let reader = BufReader::new(read_half);
        let writer = write_half;

        Connection {
            reader,
            writer,
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
        const MAX_LINE_SIZE: usize = 8193;

        let mut line = String::new();
        let mut limited_reader = BufReader::new(self.reader.get_mut()).take(MAX_LINE_SIZE as u64);
        match limited_reader.read_line(&mut line).await {
            Ok(0) => Err(ConnectionError::Closed),
            Ok(MAX_LINE_SIZE) => {
                let _ = self.ko().await;
                Err(ConnectionError::ReachedTakeLimit)
            }
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
            biased;
            res = rx.recv() => {
                let res = res.expect("Should never happen");
                match self.command_handler.handle_command(res) {
                    HandleCommandResult::Ok(str) => {
                        match self.writer.write_all(str.as_bytes()).await {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(ConnectionError::IO(e));
                            }
                        }
                    }
                    HandleCommandResult::ChangeState(response, new_state) => {
                        match new_state {
                            State::Ai => {
                                match self.writer.write_all(response.as_bytes()).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        return Err(ConnectionError::IO(e));
                                    }
                                }
                                self.command_handler = Box::new(AiHandler::new(self.command_handler.id()));
                            }
                            State::Gui => {
                                todo!("Implement GUI state");
                            }
                            _ => {
                                unreachable!("Should not land here");
                            }
                        }
                    }
                }
                Ok(())
            }

            cmd = self.read_line() => {
                let mut line = match cmd {
                    Err(ConnectionError::ReachedTakeLimit) => {
                        return Ok(()) // Ignore the line, ko has been sent
                    }
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(line) => line,
                };
                line.pop(); // Remove the trailing '\n'
                let action = self.command_handler.parse_command(line);
                match action.action {
                    Action::Ko => {
                        self.ko().await;
                    }
                    _ => {
                        tx.send(action).await.expect("Should never happen");
                    }
                }
                Ok(())
            }
        }
    }
}

impl Ko for Connection {
    async fn ko(&mut self) -> bool {
        let _ = self.writer.write_all(b"ko\n").await;
        true
    }
}
