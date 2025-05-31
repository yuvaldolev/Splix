mod shell_path_resolver;

use std::{
    ffi::CString,
    os::{
        fd::AsFd,
        unix::{
            ffi::OsStrExt,
            io::{FromRawFd, IntoRawFd},
        },
    },
};

use nix::{
    fcntl::{self, FcntlArg, OFlag},
    pty::{self, ForkptyResult},
    unistd::{self, Pid},
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};

use shell_path_resolver::ShellPathResolver;

pub struct Terminal {
    _child: Pid,
    pty: BufStream<tokio::fs::File>,
    incomplete_utf8: Vec<u8>,
}

impl Terminal {
    pub fn new() -> splix_error::Result<Self> {
        let (child, master_pty) = Self::spawn_child()?;

        let pty_flags =
            OFlag::from_bits_truncate(fcntl::fcntl(master_pty.as_fd(), FcntlArg::F_GETFL).unwrap());
        fcntl::fcntl(
            master_pty.as_fd(),
            FcntlArg::F_SETFL(pty_flags | OFlag::O_NONBLOCK),
        )
        .unwrap();

        let file = tokio::fs::File::from_std(master_pty);

        Ok(Self {
            _child: child,
            pty: BufStream::new(file),
            incomplete_utf8: Vec::new(),
        })
    }

    pub async fn read(&mut self) -> splix_error::Result<Vec<char>> {
        // Get the available bytes in the buffer
        let buffer = self
            .pty
            .fill_buf()
            .await
            .map_err(splix_error::Error::ReadFromTerminal)?;

        if buffer.is_empty() {
            return Ok(Vec::new()); // EOF
        }

        // Try to decode as much as possible
        let mut chars = Vec::new();
        let buffer_len = buffer.len();

        match std::str::from_utf8(buffer) {
            Ok(s) => {
                // Successfully decoded the entire buffer
                chars.extend(s.chars());
                self.incomplete_utf8.clear();
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to > 0 {
                    // We have some valid UTF-8, decode it
                    let s = std::str::from_utf8(&buffer[..valid_up_to]).unwrap();
                    chars.extend(s.chars());
                }

                // Check if we have an incomplete UTF-8 sequence at the end
                if valid_up_to < buffer_len {
                    // If we have an incomplete sequence from before, combine it
                    if !self.incomplete_utf8.is_empty() {
                        self.incomplete_utf8
                            .extend_from_slice(&buffer[valid_up_to..]);
                    } else {
                        // Start a new incomplete sequence
                        self.incomplete_utf8 = buffer[valid_up_to..].to_vec();
                    }
                } else {
                    self.incomplete_utf8.clear();
                }
            }
        }

        // Always consume all bytes from the buffer
        self.pty.consume(buffer_len);

        Ok(chars)
    }

    pub async fn write(&mut self, input: u8) -> splix_error::Result<()> {
        self.pty
            .get_mut()
            .write_u8(input)
            .await
            .map_err(splix_error::Error::WriteToTerminal)?;

        Ok(())
    }

    fn spawn_child() -> splix_error::Result<(Pid, std::fs::File)> {
        match Self::fork_child_process_in_pty()? {
            ForkptyResult::Parent { child, master } => {
                let file = unsafe { std::fs::File::from_raw_fd(master.into_raw_fd()) };
                Ok((child, file))
            }
            ForkptyResult::Child => {
                Self::execute_shell()?;
                Err(splix_error::Error::TerminalSpawnChild)
            }
        }
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
