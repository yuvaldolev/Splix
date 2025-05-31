mod socket;

pub use socket::{LogClient, LogServer};

use anyhow::Result;
use ringbuf::{HeapRb, Rb};
use std::{io, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::format::FmtSpan, fmt::writer::MakeWriter, prelude::*};

pub struct LogBuffer {
    pub buffer: Arc<Mutex<HeapRb<String>>>,
    server: Option<LogServer>,
}

impl LogBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(HeapRb::new(capacity))),
            server: None,
        }
    }

    pub async fn get_logs(&self) -> Vec<String> {
        let buffer = self.buffer.lock().await;
        buffer.iter().cloned().collect()
    }

    pub fn set_server(&mut self, server: LogServer) {
        self.server = Some(server);
    }
}

pub struct LogWriter {
    buffer: Arc<Mutex<HeapRb<String>>>,
    server: Option<LogServer>,
}

impl io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(log) = String::from_utf8(buf.to_vec()) {
            if let Ok(mut buffer) = self.buffer.try_lock() {
                buffer.push_overwrite(log.clone());
            }
            if let Some(server) = &self.server {
                server.broadcast_log(log);
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Clone for LogWriter {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            server: self.server.clone(),
        }
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

pub fn init_logging(log_dir: PathBuf, buffer_size: usize) -> Result<LogBuffer> {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all(&log_dir)?;

    // Configure file appender
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "splix.log");

    // Create in-memory buffer
    let buffer = LogBuffer::new(buffer_size);
    let writer = LogWriter {
        buffer: buffer.buffer.clone(),
        server: None,
    };

    // Configure the subscriber with both file and in-memory logging
    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_span_events(FmtSpan::FULL)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(writer)
                .with_span_events(FmtSpan::FULL)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        );

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tracing::{error, info};

    #[tokio::test]
    async fn test_logger_integration() -> Result<()> {
        let temp_dir = tempdir()?;
        let buffer = init_logging(temp_dir.path().to_path_buf(), 1024)?;

        // Test logging
        info!("Test info message");
        error!("Test error message");

        // Verify logs in buffer
        let logs = buffer.get_logs().await;
        assert_eq!(logs.len(), 2);
        assert!(logs[0].contains("Test info message"));
        assert!(logs[1].contains("Test error message"));

        // Verify logs in file
        let files: Vec<_> = std::fs::read_dir(temp_dir.path())?.collect();
        assert_eq!(files.len(), 1);

        Ok(())
    }
}
