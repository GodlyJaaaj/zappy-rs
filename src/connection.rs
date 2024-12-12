use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt};

pub struct Connection {
    stream: BufReader<TcpStream>,
}

#[derive(Debug)]
enum ConnectionError {
    Closed,
    IO(tokio::io::Error)
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
	    stream: BufReader::new(socket)
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

    async fn update(&mut self) -> Result<(), ConnectionError> {
	tokio::select! {
	    Ok(buf) = async {
		let mut buf = String::new();
		match self.stream.read_line(&mut buf).await {
		    Ok(0) => Err(ConnectionError::Closed),
		    Err(e) => Err(ConnectionError::IO(e)),
		    _ => Ok(buf)
		}
	    } => {
	    }
	}
	Ok(())
    }
}
