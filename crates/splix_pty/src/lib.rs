use std::fs::File;

use nix::pty;

pub struct Pty {
    master: File,
    slave: File,
}

impl Pty {
    pub fn open() -> splix_error::Result<Self> {
        let openpty_result = pty::openpty(None, None).map_err(splix_error::Error::OpenPty)?;

        Ok(Self {
            master: openpty_result.master.into(),
            slave: openpty_result.slave.into(),
        })
    }

    pub fn into_parts(self) -> (File, File) {
        (self.master, self.slave)
    }
}
