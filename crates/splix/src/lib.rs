use splix_session::Session;
use splix_termios::Termios;

pub struct Splix {
    _termios: Termios,
}

impl Splix {
    pub fn new() -> splix_error::Result<Self> {
        let termios = Termios::new()?;
        Ok(Self { _termios: termios })
    }

    pub async fn run(&self) -> splix_error::Result<()> {
        let mut sessions = vec![Session::new()?, Session::new()?];
        sessions[0].attach().await;

        Ok(())
    }
}
