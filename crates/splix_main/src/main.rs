use anyhow::Result;
use clap::{Parser, Subcommand};
use splix::Splix;
use splix_logger::{LogServer, init_logging};
use std::path::PathBuf;

const DEFAULT_SOCKET_PATH: &str = "/tmp/splix.sock";
const DEFAULT_LOG_DIR: &str = "logs";
const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024; // 1MB

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the Unix domain socket
    #[arg(long, default_value = DEFAULT_SOCKET_PATH)]
    socket_path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// View logs from a running Splix instance
    Logs {
        /// Follow logs in real-time
        #[arg(short, long)]
        follow: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Logs { follow }) => {
            // Handle logs command
            let client = splix_logger::LogClient::new(&cli.socket_path);
            client.run(follow).await?;
        }
        None => {
            // Run Splix normally
            run_splix(&cli.socket_path).await?;
        }
    }

    Ok(())
}

async fn run_splix(socket_path: &PathBuf) -> Result<()> {
    // Initialize logging
    let mut buffer = init_logging(PathBuf::from(DEFAULT_LOG_DIR), DEFAULT_BUFFER_SIZE)?;

    // Start log server
    let server = LogServer::new(socket_path.clone(), buffer.buffer.clone());
    buffer.set_server(server.clone());
    tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Log server error: {}", e);
        }
    });

    tracing::info!("Starting Splix...");
    // for i in 0..5000 {
    //     tracing::info!("i: {i}");
    //     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // }
    let mut splix = Splix::new()?;
    splix.run().await?;
    tracing::info!("Splix stopped");

    Ok(())
}
