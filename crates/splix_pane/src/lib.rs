mod grid;

pub use grid::{Grid, GridUpdate};

use tokio::sync::mpsc::Sender;

use splix_event::{Event, PaneUpdateEvent};
use splix_id::PaneId;
use splix_terminal::Terminal;

pub struct Pane {
    id: PaneId,
    grid: Grid,
}

impl Pane {
    pub fn new(id: PaneId, event_sender: Sender<Event>) -> splix_error::Result<Self> {
        let grid = Grid::new();

        let pane = Self { id, grid };

        // Clone the sender for the async task
        let task_id = pane.id;

        // Create a terminal for the async task
        let task_terminal = Terminal::new()?;

        tokio::spawn(async move {
            Self::handle_terminal_io(task_terminal, event_sender, task_id)
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
        event_sender: Sender<Event>,
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
                        event_sender
                            .send(Event::PaneUpdate(PaneUpdateEvent::new(pane_id)))
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
