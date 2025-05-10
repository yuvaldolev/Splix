use std::io;

use splix_event::Event;
use tokio::{fs::File, sync::mpsc::Sender};

pub struct InputReceiver;

impl InputReceiver {
    pub fn new(event_sender: Sender<Event>) -> Self {
        tokio::spawn(async move {
            Self::receive(event_sender);
        });

        Self
    }

    fn receive(event_sender: Sender<Event>) {
        let stdin = File::from_std(io::stdin().into());

        loop {}
    }
}
