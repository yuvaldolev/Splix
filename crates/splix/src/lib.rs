use splix_session::Session;

pub struct Splix;

impl Splix {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> splix_error::Result<()> {
        let mut sessions = vec![Session::new()?, Session::new()?];
        sessions[0].attach();

        Ok(())
    }
}
