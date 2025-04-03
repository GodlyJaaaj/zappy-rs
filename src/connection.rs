use crate::handler::ai::AiHandler;
use crate::handler::command::{CommandHandler, CommandRes, State};
use crate::handler::login::LoginHandler;
use crate::protocol::{EventType, ServerResponse, SharedAction};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

/// Manages a TCP connection with a client
pub struct Connection {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    // Channel to send events to server
    server_tx: mpsc::Sender<EventType>,
    command_handler: Box<dyn CommandHandler + Send>,
}

#[derive(Debug, Error)]
enum ConnectionError {
    #[error("Client disconnected")]
    Disconnected,
    #[error("Connection forcibly closed by server")]
    ForciblyClosedByServer,
    #[error("Failed to send event to server: {0}")]
    ServerChannelError(#[from] mpsc::error::SendError<EventType>),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Error)]
enum RecvError {
    #[error("Connection closed by client")]
    Closed,
    #[error("Client sent invalid UTF-8 data")]
    InvalidUTF8,
    #[error("Client message exceeded maximum length")]
    ReachedTakeLimit,
}

impl Connection {
    /// Creates a new client connection
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this connection
    /// * `socket` - TCP socket connected to the client
    /// * `server_tx` - Channel to send events to the server
    pub async fn new(id: u64, socket: TcpStream, server_tx: mpsc::Sender<EventType>) -> Self {
        let (read_half, write_half) = socket.into_split();
        let reader = BufReader::new(read_half);
        let mut writer = write_half;

        // Send welcome message, ignoring errors (will be handled in update loop)
        let _ = writer.write_all(b"WELCOME\n").await;

        Self {
            reader,
            writer,
            server_tx,
            command_handler: Box::new(LoginHandler::new(id)),
        }
    }

    /// Main connection handling loop
    pub async fn handle(&mut self, mut client_rx: Receiver<ServerResponse>) {
        while let Ok(_) = self.update(&mut client_rx).await {}
    }

    async fn read_line(&mut self) -> Result<String, RecvError> {
        const MAX_LINE_SIZE: usize = 8193;

        let mut line = String::new();
        let mut limited_reader = BufReader::new(self.reader.get_mut()).take(MAX_LINE_SIZE as u64);
        match limited_reader.read_line(&mut line).await {
            Ok(0) => Err(RecvError::Closed),
            Ok(MAX_LINE_SIZE) => Err(RecvError::ReachedTakeLimit),
            Ok(_) => Ok(line),
            Err(e) => Err(RecvError::InvalidUTF8),
        }
    }

    async fn send_response(&mut self, res: String) -> Result<(), ConnectionError> {
        self.writer.write_all(res.as_bytes()).await?;
        Ok(())
    }

    async fn handle_recv_error(&mut self, e: &RecvError) -> Result<(), ConnectionError> {
        let action = match e {
            RecvError::Closed => SharedAction::Disconnected,
            RecvError::InvalidUTF8 => SharedAction::InvalidEncoding,
            RecvError::ReachedTakeLimit => SharedAction::ReachedTakeLimit,
        };

        self.server_tx
            .send(self.command_handler.create_shared_event(action))
            .await?;

        if matches!(e, RecvError::Closed) {
            Err(ConnectionError::Disconnected)
        } else {
            Ok(())
        }
    }

    async fn update(
        &mut self,
        client_rx: &mut Receiver<ServerResponse>,
    ) -> Result<(), ConnectionError> {
        tokio::select! {
            biased;

            Some(response) = client_rx.recv() => {
                match self.command_handler.handle_command(response) {
                    CommandRes::ChangeState(State::IA(res)) => {
                        self.command_handler = Box::new(AiHandler::new(self.command_handler.id()));
                        self.send_response(res).await?;
                    }
                    CommandRes::ChangeState(State::GUI(_)) => {
                        // TODO: Implement GUI state handling
                        return Err(ConnectionError::ForciblyClosedByServer);
                    }
                    CommandRes::Response(res) => {
                        self.send_response(res).await?;
                    },
                    CommandRes::ChangeState(State::DEAD(res)) => {
                        self.send_response(res).await?;
                        return Err(ConnectionError::ForciblyClosedByServer);
                    }
                }
                Ok(())
            }

            cmd_result = self.read_line() => {
                let line = match cmd_result {
                    Ok(mut line) => {
                        line.pop(); // Remove the trailing '\n'
                        line
                    }
                    Err(e) => return self.handle_recv_error(&e).await,
                };

                let action = self.command_handler.parse_command(line);
                self.server_tx.send(action).await?;
                Ok(())
            }
        }
    }
}
