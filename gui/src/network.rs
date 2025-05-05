use futures::channel::mpsc;
use futures::{SinkExt, Stream, StreamExt};
use iced_futures::stream;
use log::{error, info, warn};
use std::net::SocketAddrV4;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::time::timeout;

#[derive(Clone, Debug)]
pub enum NetworkOutput {
    Ready(mpsc::Sender<NetworkInput>),
    Connected(SocketAddrV4, mpsc::Sender<GuiToServerMessage>),
    Disconnected,
    ConnectionFailed(SocketAddrV4, String),
    ServerMessage(ServerMessage),
}

pub enum NetworkInput {
    Connect(SocketAddrV4),
    Disconnect,
}

pub enum GuiToServerMessage {}

#[derive(Clone, Debug)]
pub enum ServerMessage {
    MapSize { width: u32, height: u32 },
    TeamName { name: String },
    Other(()), // For any other messages
}

fn parse_server_message(msg: &str) -> Option<ServerMessage> {
    let parts: Vec<&str> = msg.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "msz" => {
            if parts.len() >= 3 {
                if let (Ok(width), Ok(height)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>())
                {
                    return Some(ServerMessage::MapSize {
                        width: width,
                        height: height,
                    });
                }
            }
        }
        "tna" => {
            if parts.len() >= 2 {
                return Some(ServerMessage::TeamName {
                    name: parts[1].to_string(),
                });
            }
        }
        _ => return Some(ServerMessage::Other(())),
    }
    None
}

async fn handle_connection(
    addr: SocketAddrV4,
    mut output_clone: mpsc::Sender<NetworkOutput>,
    cmd_sender: mpsc::Sender<GuiToServerMessage>,
    mut cmd_receiver: mpsc::Receiver<GuiToServerMessage>,
) {
    let timeout_duration = Duration::from_secs(5);

    match timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(mut s)) => {
            let _ = s.write_all(b"GRAPHIC\n").await;
            let _ = output_clone.try_send(NetworkOutput::Connected(addr, cmd_sender));
            tokio::time::sleep(Duration::from_millis(500)).await;
            let _ = s.write_all(b"msz\n").await;
            let _ = s.write_all(b"tna\n").await;

            let mut buffer = [0u8; 1024];
            loop {
                select! {
                    result = s.read(&mut buffer) => {
                        match result {
                            Ok(0) => {
                                info!("Connection closed by server");
                                let  _ = output_clone.try_send(NetworkOutput::Disconnected);
                                break;
                            }
                            Ok(n) => {
                                let received = buffer.iter().take(n.saturating_sub(1)).map(|b| *b as char).collect::<String>();
                                info!("Got {} bytes from server : [{}]", n, received);

                                // Process each line separately
                                for line in received.lines() {
                                    if let Some(parsed_msg) = parse_server_message(line) {
                                        info!("Parsed message: {:?}", parsed_msg);
                                        // Forward the parsed message to the GUI
                                        let _ = output_clone.try_send(NetworkOutput::ServerMessage(parsed_msg));
                                    }
                                }
                            }

                            Err(e) => {
                                error!("Failed to read from server: {}", e);
                                let _ = output_clone.try_send(NetworkOutput::Disconnected);
                                break;
                            }
                        }
                    }
                    cmd = cmd_receiver.select_next_some() => {
                        match cmd {
                        }
                    }
                }
            }
        }
        Err(_) | Ok(Err(_)) => {
            warn!("Failed to connect to server");
            let _ = output_clone.try_send(NetworkOutput::ConnectionFailed(
                addr,
                "Cannot connect to server.".to_string(),
            ));
        }
    }
}

pub fn network_worker() -> impl Stream<Item = NetworkOutput> {
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = mpsc::channel(100);
        let _ = output.send(NetworkOutput::Ready(sender)).await;

        let mut current_connection: Option<tokio::task::JoinHandle<()>> = None;

        loop {
            let input = receiver.select_next_some().await;
            match input {
                NetworkInput::Connect(addr) => {
                    if let Some(handle) = current_connection.take() {
                        handle.abort();
                    }

                    let (cmd_sender, cmd_receiver) = mpsc::channel(100);

                    let output_clone = output.clone();

                    let task = tokio::spawn(handle_connection(
                        addr,
                        output_clone,
                        cmd_sender,
                        cmd_receiver,
                    ));
                    current_connection = Some(task);
                }
                NetworkInput::Disconnect => {
                    if let Some(handle) = current_connection.take() {
                        handle.abort();
                        let _ = output.try_send(NetworkOutput::Disconnected);
                    }
                }
            }
        }
    })
}
