use tokio::{
    io::{self, AsyncReadExt},
    sync::mpsc::Sender,
};

use splix_event::Event;

pub struct InputReceiver;

impl InputReceiver {
    pub fn new(event_sender: Sender<Event>) -> Self {
        tokio::spawn(async move {
            Self::receive(event_sender).await;
        });

        Self
    }

    async fn receive(event_sender: Sender<Event>) {
        let mut stdin = io::stdin();

        loop {
            event_sender
                .send(Event::Input(stdin.read_u8().await.unwrap()))
                .await
                .unwrap();
        }
    }
}
