use splix_id::SessionId;
use splix_session::Session;
use splix_termios::Termios;

pub struct Splix {
    _termios: Termios,
    sessions: Vec<Session>,
    next_session_id: usize,
}

impl Splix {
    pub fn new() -> splix_error::Result<Self> {
        let termios = Termios::new()?;

        let mut splix = Self {
            _termios: termios,
            sessions: Vec::new(),
            next_session_id: 0,
        };

        splix.new_session()?;

        Ok(splix)
    }

    pub async fn run(&mut self) -> splix_error::Result<()> {
        self.sessions[0].attach().await;

        Ok(())
    }

    fn new_session(&mut self) -> splix_error::Result<()> {
        let id = SessionId::new(self.next_session_id);
        let session = Session::new(id)?;
        self.sessions.push(session);
        self.next_session_id += 1;

        Ok(())
    }
}
