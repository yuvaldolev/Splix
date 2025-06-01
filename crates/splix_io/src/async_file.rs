use std::{
    io::Read,
    os::fd::{AsFd, AsRawFd},
};

use nix::fcntl::{self, FcntlArg, OFlag};
use tokio::io::unix::AsyncFd;

pub struct AsyncFile<T: AsRawFd> {
    fd: AsyncFd<T>,
}

impl<T> AsyncFile<T>
where
    T: AsFd + AsRawFd + Read,
{
    pub fn new(file: T) -> splix_error::Result<Self> {
        Self::set_non_blocking(&file)?;

        let raw_fd = file.as_raw_fd();
        let fd =
            AsyncFd::new(file).map_err(|e| splix_error::Error::CreateAsyncFdForFile(e, raw_fd))?;

        Ok(Self { fd })
    }

    pub async fn read(&mut self, buffer: &mut [u8]) -> splix_error::Result<usize> {
        loop {
            if let Some(bytes_read) = self.read_once(buffer).await? {
                return Ok(bytes_read);
            }
        }
    }

    fn set_non_blocking(file: &T) -> splix_error::Result<()> {
        let open_flags = OFlag::from_bits_truncate(fcntl::fcntl(file, FcntlArg::F_GETFL).unwrap());
        fcntl::fcntl(file, FcntlArg::F_SETFL(open_flags | OFlag::O_NONBLOCK))
            .map_err(|e| splix_error::Error::MakeFileNonBlocking(e, file.as_raw_fd()))?;

        Ok(())
    }

    async fn read_once(&mut self, buffer: &mut [u8]) -> splix_error::Result<Option<usize>> {
        let raw_fd = self.fd.get_ref().as_raw_fd();

        let mut guard = self
            .fd
            .readable_mut()
            .await
            .map_err(|e| splix_error::Error::WaitForFileToBecomeReadable(e, raw_fd))?;

        let result = match guard.try_io(|inner| inner.get_mut().read(buffer)) {
            Ok(result) => Some(result.map_err(|e| splix_error::Error::ReadFromFile(e, raw_fd))?),
            Err(_) => None,
        };

        Ok(result)
    }
}
