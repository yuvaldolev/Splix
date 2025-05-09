use tokio::sync::mpsc::{self, Receiver, Sender};

use splix_event::Event;
use splix_id::SessionId;
use splix_session::Session;
use splix_termios::Termios;

pub struct Splix {
    _termios: Termios,
    sessions: Vec<Session>,
    next_session_id: usize,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
}

const EVENT_CHANNEL_CAPACITY: usize = 1024;

impl Splix {
    pub fn new() -> splix_error::Result<Self> {
        let termios = Termios::new()?;
        let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) =
            mpsc::channel(EVENT_CHANNEL_CAPACITY);

        let mut splix = Self {
            _termios: termios,
            sessions: Vec::new(),
            next_session_id: 0,
            event_sender,
            event_receiver,
        };

        splix.new_session()?;

        Ok(splix)
    }

    pub async fn run(&mut self) -> splix_error::Result<()> {
        while let Some(event) = self.event_receiver.recv().await {
            println!("Got event: {event:?}");
        }

        Ok(())
    }

    fn new_session(&mut self) -> splix_error::Result<()> {
        let id = SessionId::new(self.next_session_id);
        let session = Session::new(id, self.event_sender.clone())?;
        self.sessions.push(session);
        self.next_session_id += 1;

        Ok(())
    }
}
