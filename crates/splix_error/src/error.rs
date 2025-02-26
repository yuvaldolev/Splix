use std::io;

use nix::errno::Errno;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed opening a PTY")]
    OpenPty(#[source] Errno),

    #[error("failed retrieving TTY termios")]
    RetrieveTtyTermios(#[source] Errno),

    #[error("failed setting TTY termios")]
    SetTtyTermios(#[source] Errno),

    #[error("enter alternate terminal screen")]
    EnterAlternateTerminalScreen(#[source] io::Error),
}
