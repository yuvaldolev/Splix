mod splix_service;

use std::{
    fs,
    path::{Path, PathBuf},
};

use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport;

use splix_api::SplixApiServer;
use splix_service::SplixService;

pub struct Server;

impl Server {
    pub fn new(socket_path: PathBuf) -> Self {
        tokio::spawn(Self::serve(socket_path));

        Self
    }

    async fn serve(socket_path: PathBuf) -> splix_error::Result<()> {
        transport::Server::builder()
            .add_service(SplixApiServer::new(SplixService::new()))
            .serve_with_incoming(Self::make_unix_domain_socket(&socket_path)?)
            .await
            .unwrap();

        Ok(())
    }

    fn make_unix_domain_socket(path: &Path) -> splix_error::Result<UnixListenerStream> {
        fs::remove_file(path).ok();

        let socket = UnixListener::bind(path)
            .map_err(|e| splix_error::Error::BindUnixDomainSocket(e, path.to_owned()))?;

        Ok(UnixListenerStream::new(socket))
    }
}
