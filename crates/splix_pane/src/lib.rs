mod grid;

pub use grid::{Grid, GridUpdate};

use splix_id::PaneId;
use splix_terminal::Terminal;
use tokio::sync::mpsc::Sender;

pub struct PaneUpdate {
    pub pane_id: PaneId,
    pub update: GridUpdate,
}

pub struct Pane {
    pub id: PaneId,
    pub grid: Grid,
    sender: Sender<PaneUpdate>,
}

impl Pane {
    pub fn new(id: PaneId, sender: Sender<PaneUpdate>) -> splix_error::Result<Self> {
        let grid = Grid::new();

        let pane = Self { id, grid, sender };

        // Clone the sender for the async task
        let task_sender = pane.sender.clone();
        let task_id = pane.id;

        // Create a terminal for the async task
        let task_terminal = Terminal::new()?;

        tokio::spawn(async move {
            Self::handle_terminal_io(task_terminal, task_sender, task_id)
                .await
                .ok();
        });

        Ok(pane)
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    async fn handle_terminal_io(
        mut terminal: Terminal,
        sender: Sender<PaneUpdate>,
        pane_id: PaneId,
    ) -> splix_error::Result<()> {
        loop {
            match terminal.read().await {
                Ok(chars) => {
                    if chars.is_empty() {
                        // EOF reached
                        break;
                    }

                    for ch in chars {
                        let update = if ch == '\n' {
                            GridUpdate::NewLine
                        } else {
                            GridUpdate::AppendChar(ch)
                        };
                        sender
                            .send(PaneUpdate { pane_id, update })
                            .await
                            .map_err(|_| splix_error::Error::SendPaneUpdate)?;
                    }
                }
                Err(_) => {
                    // log::error!("Failed to read from terminal: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
