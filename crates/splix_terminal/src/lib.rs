mod shell_path_resolver;

use std::{ffi::CString, os::unix::ffi::OsStrExt};

use nix::{
    pty::{self, ForkptyResult},
    unistd::{self, Pid},
};

use shell_path_resolver::ShellPathResolver;
use tokio::io::AsyncReadExt;

pub struct Terminal {
    _child: Pid,
    master_pty: tokio::fs::File,
}

impl Terminal {
    pub fn new() -> splix_error::Result<Self> {
        let (child, master_pty) = Self::spawn_child()?;

        Ok(Self {
            _child: child,
            master_pty,
        })
    }

    pub async fn read(&mut self) -> u8 {
        self.master_pty.read_u8().await.unwrap()
    }

    fn spawn_child() -> splix_error::Result<(Pid, tokio::fs::File)> {
        match Self::fork_child_process_in_pty()? {
            ForkptyResult::Parent { child, master } => {
                return Ok((child, tokio::fs::File::from(std::fs::File::from(master))))
            }
            ForkptyResult::Child => Self::execute_shell()?,
        }

        Err(splix_error::Error::TerminalSpawnChild)
    }

    fn fork_child_process_in_pty() -> splix_error::Result<ForkptyResult> {
        unsafe { pty::forkpty(None, None) }.map_err(splix_error::Error::ForkChildProcessInPty)
    }

    fn execute_shell() -> splix_error::Result<()> {
        let shell_path_resolver = ShellPathResolver::new();

        let shell_path = shell_path_resolver.resolve();
        let shell_path_c_string = CString::new(shell_path.as_os_str().as_bytes())
            .expect("CString should be successfully created");

        unistd::execv(&shell_path_c_string, &[&shell_path_c_string])
            .expect("shell should be executed");

        Ok(())
    }
}
