use std::os::fd::AsFd;

use nix::fcntl::{self, FcntlArg, OFlag};
use tokio::{
    io::{self, AsyncReadExt},
    sync::mpsc::Sender,
};

use splix_event::Event;

pub struct InputReceiver;

impl InputReceiver {
    pub fn new(event_sender: Sender<Event>) -> Self {
        tokio::spawn(Self::receive(event_sender));

        Self
    }

    async fn receive(event_sender: Sender<Event>) {
        let mut stdin = io::stdin();

        let pty_flags =
            OFlag::from_bits_truncate(fcntl::fcntl(stdin.as_fd(), FcntlArg::F_GETFL).unwrap());
        fcntl::fcntl(
            stdin.as_fd(),
            FcntlArg::F_SETFL(pty_flags | OFlag::O_NONBLOCK),
        )
        .unwrap();

        loop {
            event_sender
                .send(Event::Input(stdin.read_u8().await.unwrap()))
                .await
                .unwrap();
        }
    }
}
