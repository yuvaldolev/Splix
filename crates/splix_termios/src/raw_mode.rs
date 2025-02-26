use std::io::{self, Stdin};

use nix::sys::termios::{self, SetArg, Termios};

pub struct RawMode {
    tty: Stdin,
    original_termios: Termios,
}

impl RawMode {
    pub fn new() -> splix_error::Result<Self> {
        // TODO: Handle cases where stdin is not a TTY.
        let tty = io::stdin();

        let original_termios =
            termios::tcgetattr(&tty).map_err(splix_error::Error::RetrieveTtyTermios)?;
        let mut raw_termios = original_termios.clone();
        termios::cfmakeraw(&mut raw_termios);
        termios::tcsetattr(&tty, SetArg::TCSANOW, &raw_termios)
            .map_err(splix_error::Error::SetTtyTermios)?;

        Ok(Self {
            tty,
            original_termios,
        })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        termios::tcsetattr(&self.tty, SetArg::TCSANOW, &self.original_termios).ok();
    }
}
