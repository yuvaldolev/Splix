use nix::errno::Errno;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed opening a PTY")]
    OpenPty(#[source] Errno),
}
