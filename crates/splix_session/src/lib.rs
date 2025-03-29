use std::io;

use byteorder::WriteBytesExt;
use splix_terminal::Terminal;

pub struct Session {
    terminal: Terminal,
}

impl Session {
    pub fn new() -> splix_error::Result<Self> {
        let terminal = Terminal::new()?;

        Ok(Self { terminal })
    }

    pub async fn attach(&mut self) {
        loop {
            let byte = self.terminal.read().await;
            io::stdout().write_u8(byte).unwrap();
        }
    }
}
