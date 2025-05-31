use anyhow::{Context, Result};
use ringbuf::{HeapRb, Rb};
use std::{path::PathBuf, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::{Mutex, broadcast},
};

#[repr(u64)]
enum Request {
    Unknown = 0,
    GetAll = 1,
    Follow = 2,
}

impl From<u64> for Request {
    fn from(value: u64) -> Self {
        match value {
            1 => Request::GetAll,
            2 => Request::Follow,
            _ => Request::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct LogServer {
    socket_path: PathBuf,
    buffer: Arc<Mutex<HeapRb<String>>>,
    log_sender: broadcast::Sender<String>,
}

impl LogServer {
    pub fn new(socket_path: PathBuf, buffer: Arc<Mutex<HeapRb<String>>>) -> Self {
        let (log_sender, _) = broadcast::channel(1024); // Buffer size for the channel
        Self {
            socket_path,
            buffer,
            log_sender,
        }
    }

    pub async fn start(&self) -> Result<()> {
        // Remove existing socket file if it exists
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        let listener = UnixListener::bind(&self.socket_path)
            .with_context(|| format!("Failed to bind to socket: {:?}", self.socket_path))?;

        loop {
            let (mut socket, _) = listener.accept().await?;
            let buffer = self.buffer.clone();
            let log_sender = self.log_sender.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(&mut socket, buffer, log_sender).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        socket: &mut UnixStream,
        buffer: Arc<Mutex<HeapRb<String>>>,
        log_sender: broadcast::Sender<String>,
    ) -> Result<()> {
        let request = Request::from(socket.read_u64().await?);

        // Send existing logs
        let logs = {
            let buffer = buffer.lock().await;
            buffer.iter().cloned().collect::<Vec<_>>()
        };

        for log in logs {
            socket.write_all(log.as_bytes()).await?;
            socket.write_all(b"\n").await?;
        }

        // If it's a follow request, start streaming new logs
        match request {
            Request::Follow => {
                println!("follow request");
                let mut receiver = log_sender.subscribe();

                loop {
                    match receiver.recv().await {
                        Ok(log) => {
                            println!("writing log");
                            socket.write_all(log.as_bytes()).await?;
                            socket.write_all(b"\n").await?;
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            // If we lagged behind, just continue with new logs
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            // Channel closed, exit the loop
                            break;
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    pub fn broadcast_log(&self, log: String) {
        let _ = self.log_sender.send(log);
    }
}

pub struct LogClient {
    socket_path: PathBuf,
}

impl LogClient {
    pub fn new(socket_path: &PathBuf) -> Self {
        Self {
            socket_path: socket_path.clone(),
        }
    }

    pub async fn run(&self, follow: bool) -> Result<()> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .with_context(|| format!("Failed to connect to socket: {:?}", self.socket_path))?;

        // Send request type
        let request = if follow {
            Request::Follow
        } else {
            Request::GetAll
        };
        stream.write_u64(request as u64).await?;

        // Read and print logs
        let mut buf = [0; 1024];
        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                if !follow {
                    break;
                }
                continue;
            }
            print!("{}", String::from_utf8_lossy(&buf[..n]));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_log_server_client() -> Result<()> {
        let temp_dir = tempdir()?;
        let socket_path = temp_dir.path().join("test.sock");
        let buffer = Arc::new(Mutex::new(HeapRb::new(1024)));

        // Add some test logs
        {
            let mut buffer = buffer.lock().await;
            buffer.push_overwrite("Test log 1".to_string());
            buffer.push_overwrite("Test log 2".to_string());
        }

        // Start server
        let server = LogServer::new(socket_path.clone(), buffer.clone());
        let server_handle = tokio::spawn(async move {
            server.start().await.unwrap();
        });

        // Wait for server to start
        sleep(Duration::from_millis(100)).await;

        // Test client
        let client = LogClient::new(&socket_path);
        let client_result = client.run(false).await;
        assert!(client_result.is_ok());

        // Cleanup
        server_handle.abort();
        Ok(())
    }

    #[tokio::test]
    async fn test_follow_mode() -> Result<()> {
        let temp_dir = tempdir()?;
        let socket_path = temp_dir.path().join("test.sock");
        let buffer = Arc::new(Mutex::new(HeapRb::new(1024)));

        // Start server
        let server = LogServer::new(socket_path.clone(), buffer.clone());
        let server_clone = server.clone();
        let server_handle = tokio::spawn(async move {
            server.start().await.unwrap();
        });

        // Wait for server to start
        sleep(Duration::from_millis(100)).await;

        // Start client in follow mode
        let client = LogClient::new(&socket_path);
        let client_handle = tokio::spawn(async move {
            client.run(true).await.unwrap();
        });

        // Send some logs
        server_clone.broadcast_log("New log 1".to_string());
        sleep(Duration::from_millis(100)).await;
        server_clone.broadcast_log("New log 2".to_string());

        // Cleanup
        server_handle.abort();
        client_handle.abort();
        Ok(())
    }
}
