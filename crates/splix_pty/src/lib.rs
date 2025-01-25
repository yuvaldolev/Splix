use std::fs::File;

use nix::pty;

pub struct Pty {
    master: File,
    slave: File,
}

impl Pty {
    pub fn open() -> Self {
        // TODO: Handle errors.
        let openpty_result = pty::openpty(None, None).unwrap();
        Self {
            master: openpty_result.master.into(),
            slave: openpty_result.slave.into(),
        }
    }

    pub fn into_parts(self) -> (File, File) {
        (self.master, self.slave)
    }
}
