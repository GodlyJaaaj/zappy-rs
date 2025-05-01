use futures::channel::mpsc;
use futures::{SinkExt, Stream, StreamExt};
use iced_futures::stream;
use log::{error, info, warn};
use std::net::SocketAddrV4;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

#[derive(Clone, Debug)]
pub enum NetworkOutput {
    Ready(mpsc::Sender<NetworkInput>),
    Connected(SocketAddrV4, mpsc::Sender<GuiToServerMessage>),
    Disconnected,
    ConnectionFailed(SocketAddrV4, String),
}

pub enum NetworkInput {
    Connect(SocketAddrV4),
    Disconnect,
}

pub enum GuiToServerMessage {
    // Add your message types here
}

async fn handle_connection(
    addr: SocketAddrV4,
    mut output_clone: mpsc::Sender<NetworkOutput>,
    cmd_sender: mpsc::Sender<GuiToServerMessage>,
    mut cmd_receiver: mpsc::Receiver<GuiToServerMessage>,
) {
    match TcpStream::connect(addr).await {
        Ok(mut s) => {
            let _ = s.write_all(b"GRAPHIC\n").await;
            let _ = output_clone.try_send(NetworkOutput::Connected(addr, cmd_sender));

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
                                info!("Got {} bytes from server", n);
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
        Err(e) => {
            warn!("Failed to connect to server : {}", e);
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
