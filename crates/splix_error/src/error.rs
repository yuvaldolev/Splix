use std::{io, path::PathBuf};

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

    #[error("failed forking a new child process in a PTY")]
    ForkChildProcessInPty(#[source] Errno),

    #[error("failed spawning a terminal child process - WTF???")]
    TerminalSpawnChild,

    #[error("failed reading from terminal")]
    ReadFromTerminal(#[source] io::Error),

    #[error("failed writing to terminal")]
    WriteToTerminal(#[source] io::Error),

    #[error("failed sending pane update")]
    SendPaneUpdate,

    #[error("failed retrieving the terminal size")]
    RetrieveTerminalSize,

    #[error("failed binding unix domain socket at path '{1}'")]
    BindUnixDomainSocket(#[source] io::Error, PathBuf),
}
