use std::io::{self, Stdout, Write};

use splix_ansi::AnsiEncoder;

pub struct AlternateScreen {
    ansi_encoder: AnsiEncoder,
    tty: Stdout,
}

const ENTER_ANSI_ESCAPE_CODE: &str = "?1049h";
const LEAVE_ANSI_ESCAPE_CODE: &str = "?1049l";

impl AlternateScreen {
    pub fn new() -> splix_error::Result<Self> {
        let ansi_encoder = AnsiEncoder::new();

        // TODO: Handle cases where stdout is not a TTY.
        let mut tty = io::stdout();
        tty.write_all(ansi_encoder.encode(ENTER_ANSI_ESCAPE_CODE).as_bytes())
            .map_err(splix_error::Error::EnterAlternateTerminalScreen)?;

        Ok(Self { ansi_encoder, tty })
    }
}

impl Drop for AlternateScreen {
    fn drop(&mut self) {
        self.tty
            .write_all(self.ansi_encoder.encode(LEAVE_ANSI_ESCAPE_CODE).as_bytes())
            .ok();
    }
}
