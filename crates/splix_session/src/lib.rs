use splix_window::Window;

pub struct Session {
    windows: Vec<Window>,
}

impl Session {
    pub fn new() -> splix_error::Result<Self> {
        Ok(Self {
            windows: vec![Window::new()],
        })
    }

    pub async fn attach(&mut self) {
        // loop {
        // let byte = self.terminal.read().await;
        // io::stdout().write_u8(byte).unwrap();
        // }
    }
}
