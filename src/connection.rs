use crate::constant::MAX_LINE_SIZE;
use crate::handler::ai::AiHandler;
use crate::handler::command::{CommandHandler, CommandRes, State};
use crate::handler::graphics::GraphicHandler;
use crate::handler::login::LoginHandler;
use crate::protocol::{EventType, ServerResponse, SharedAction};
use log::{debug, error, warn};
use std::time::Duration;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tokio::time::timeout;

/// Manages a TCP connection with a client
pub struct Connection {
    writer: OwnedWriteHalf,
    // Channel to send events to server
    server_tx: mpsc::Sender<EventType>,
    command_handler: Box<dyn CommandHandler + Send>,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Client disconnected")]
    Disconnected,
    #[error("Connection forcibly closed by server")]
    ForciblyClosedByServer,
    #[error("Failed to send event to server: {0}")]
    ServerChannelError(#[from] mpsc::error::SendError<EventType>),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Timeout")]
    Timeout,
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

enum ConnectionEvent {
    ClientMessage(String),
    ClientError(RecvError),
    ServerResponse(ServerResponse),
}

impl Connection {
    /// Creates a new client connection
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this connection
    /// * `socket` - TCP socket connected to the client
    /// * `server_tx` - Channel to send events to the server
    pub async fn new(
        id: u64,
        socket: TcpStream,
        server_tx: mpsc::Sender<EventType>,
    ) -> (Self, BufReader<OwnedReadHalf>) {
        let (read_half, write_half) = socket.into_split();
        let mut writer = write_half;

        // Send welcome message, ignoring errors (will be handled in update loop)
        let _ = writer.write_all(b"WELCOME\n").await;

        (
            Self {
                writer,
                server_tx,
                command_handler: Box::new(LoginHandler::new(id)),
            },
            BufReader::new(read_half),
        )
    }

    /// Main connection handling loop
    pub async fn handle(
        &mut self,
        client_rx: Receiver<ServerResponse>,
        reader_half: BufReader<OwnedReadHalf>,
    ) -> Result<(), ConnectionError> {
        let (event_tx, mut event_rx) = mpsc::channel::<ConnectionEvent>(32);

        let reader_task = self.spawn_reader_task(reader_half, event_tx.clone());

        let server_task = self.spawn_server_task(client_rx, event_tx);

        let mut result = Ok(());

        'main: while let Some(event) = event_rx.recv().await {
            match event {
                ConnectionEvent::ClientMessage(line) => {
                    let line = line.trim_end();

                    let action = self.command_handler.parse_command(line.to_string());
                    let _ = self.server_tx.send(action).await;
                }
                ConnectionEvent::ClientError(err) => {
                    match &err {
                        RecvError::Closed => {
                            warn!("Client {}: Connection closed", self.command_handler.id());
                        }
                        RecvError::InvalidUTF8 => {
                            warn!("Client {}: Invalid UTF-8 data", self.command_handler.id());
                            let _ = self
                                .server_tx
                                .send(
                                    self.command_handler
                                        .create_shared_event(SharedAction::InvalidEncoding),
                                )
                                .await;
                        }
                        RecvError::ReachedTakeLimit => {
                            warn!("Client {}: Message too long", self.command_handler.id());
                            let _ = self
                                .server_tx
                                .send(
                                    self.command_handler
                                        .create_shared_event(SharedAction::ReachedTakeLimit),
                                )
                                .await;
                        }
                    };

                    if matches!(err, RecvError::Closed) {
                        result = Err(ConnectionError::Disconnected);
                        break 'main;
                    }
                }
                ConnectionEvent::ServerResponse(response) => {
                    match self.command_handler.handle_command(response) {
                        CommandRes::ChangeState(State::IA(res)) => {
                            self.command_handler =
                                Box::new(AiHandler::new(self.command_handler.id()));
                            if let Err(e) = self.send_response_with_timeout(res).await {
                                error!(
                                    "Client {}: Failed to send response: {}",
                                    self.command_handler.id(),
                                    e
                                );
                                result = Err(e);
                                break 'main;
                            }
                        }
                        CommandRes::ChangeState(State::GUI) => {
                            self.command_handler =
                                Box::new(GraphicHandler::new(self.command_handler.id()));
                        }
                        CommandRes::Response(res) => {
                            if let Err(e) = self.send_response_with_timeout(res).await {
                                error!(
                                    "Client {}: Failed to send response: {}",
                                    self.command_handler.id(),
                                    e
                                );
                                result = Err(e);
                                break 'main;
                            }
                        }
                        CommandRes::ChangeState(State::DEAD(res)) => {
                            let _ = self.send_response_with_timeout(res).await;
                            result = Err(ConnectionError::ForciblyClosedByServer);
                            break 'main;
                        }
                    }
                }
            }
        }

        reader_task.abort();
        server_task.abort();

        if result.is_err() {
            self.server_tx
                .send(
                    self.command_handler
                        .create_shared_event(SharedAction::Disconnected),
                )
                .await?;
        }
        result
    }

    /// Spawn a task that reads from the client socket
    fn spawn_reader_task(
        &self,
        mut reader_half: BufReader<OwnedReadHalf>,
        event_tx: mpsc::Sender<ConnectionEvent>,
    ) -> JoinHandle<()> {
        let client_id = self.command_handler.id();

        async fn read_line(reader_half: &mut  BufReader<OwnedReadHalf>) -> Result<String, RecvError> {
            let mut line = String::new();
            match reader_half.read_line(&mut line).await {
                Ok(0) => Err(RecvError::Closed),
                Ok(n) if n > MAX_LINE_SIZE => Err(RecvError::ReachedTakeLimit),
                Ok(_) => Ok(line),
                Err(_) => Err(RecvError::InvalidUTF8),
            }
        }

        tokio::spawn(async move {
            loop {
                match read_line(&mut reader_half).await {
                    Ok(line) => {
                        if event_tx
                            .send(ConnectionEvent::ClientMessage(line))
                            .await
                            .is_err()
                        {
                            debug!("Client {}: Reader task channel closed", client_id);
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = event_tx.send(ConnectionEvent::ClientError(e.clone())).await;
                        if matches!(e, RecvError::Closed) {
                            debug!(
                                "Client {}: Connection closed, reader task exiting",
                                client_id
                            );
                            break;
                        }
                    }
                }
            }
        })
    }

    /// Spawn a task that receives messages from the server
    fn spawn_server_task(
        &self,
        mut client_rx: Receiver<ServerResponse>,
        event_tx: mpsc::Sender<ConnectionEvent>,
    ) -> JoinHandle<()> {
        let client_id = self.command_handler.id();

        tokio::spawn(async move {
            while let Some(response) = client_rx.recv().await {
                if event_tx
                    .send(ConnectionEvent::ServerResponse(response))
                    .await
                    .is_err()
                {
                    debug!("Client {}: Server task channel closed", client_id);
                    break;
                }
            }
            debug!("Client {}: Server channel closed, task exiting", client_id);
        })
    }

    async fn send_response_with_timeout(&mut self, res: String) -> Result<(), ConnectionError> {
        let writer = &mut self.writer;

        timeout(Duration::from_secs(5), async {
            writer.write_all(res.as_bytes()).await?;
            Ok(())
        })
        .await
        .unwrap_or(Err(ConnectionError::Timeout))
    }
}
