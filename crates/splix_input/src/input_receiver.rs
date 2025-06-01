use std::io::{self};

use splix_event::Event;
use splix_io::AsyncFile;
use tokio::sync::mpsc::Sender;

pub struct InputReceiver;

const READ_BUFFER_SIZE: usize = 1024;

impl InputReceiver {
    pub fn new(event_sender: Sender<Event>) -> Self {
        tokio::spawn(Self::receive(event_sender));

        Self
    }

    async fn receive(event_sender: Sender<Event>) -> splix_error::Result<()> {
        let mut stdin = AsyncFile::new(io::stdin())?;
        let mut buffer: [u8; READ_BUFFER_SIZE] = [0; READ_BUFFER_SIZE];

        loop {
            let bytes_read = stdin.read(&mut buffer).await?;

            // TODO: Is stdin closing an error?
            if 0 == bytes_read {
                break;
            }

            for byte in &buffer[0..bytes_read] {
                event_sender.send(Event::Input(*byte)).await.unwrap()
            }
        }

        Ok(())
    }
}
