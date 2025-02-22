use std::{
    fs::File,
    io,
    process::{Child, Command},
};

use byteorder::{ReadBytesExt, WriteBytesExt};
use nix::sys::termios::{self, SetArg};
use splix_pty::Pty;

pub struct Session {
    pty_master: File,
    shell: Child,
}

impl Session {
    pub fn new() -> splix_error::Result<Self> {
        // TODO: Export all terminal related handling stuff into a dedicated struct.
        let pty = Pty::open()?;
        let (master, slave) = pty.into_parts();

        // TODO: Handle errors.
        let shell = Command::new("/bin/bash")
            .stdin(slave.try_clone().unwrap())
            .stdout(slave.try_clone().unwrap())
            .stderr(slave)
            .spawn()
            .unwrap();

        let mut raw_termios = termios::tcgetattr(&master).unwrap();
        termios::cfmakeraw(&mut raw_termios);
        termios::tcsetattr(&master, SetArg::TCSANOW, &raw_termios).unwrap();

        Ok(Self {
            pty_master: master,
            shell,
        })
    }

    pub fn attach(&mut self) {
        loop {
            let byte = self.pty_master.read_u8().unwrap();
            io::stdout().write_u8(byte).unwrap();
        }
    }
}
