mod grid;

use std::error::Error;

use grid::Grid;
use tokio::sync::mpsc::{self, Receiver, Sender};

use splix_event::{Event, GridUpdate, PaneUpdateEvent};
use splix_id::PaneId;
use splix_terminal::Terminal;

pub struct Pane {
    id: PaneId,
    grid: Grid,
    input_sender: Sender<u8>,
}

impl Pane {
    pub fn new(id: PaneId, event_sender: Sender<Event>) -> splix_error::Result<Self> {
        let grid = Grid::new();
        let (input_sender, input_receiver): (Sender<u8>, Receiver<u8>) = mpsc::channel(32);
        let pane = Self {
            id,
            grid,
            input_sender,
        };

        // Clone the sender for the async task
        let task_id = pane.id;

        // Create a terminal for the async task
        let task_terminal = Terminal::new()?;

        tokio::spawn(async move {
            if let Err(e) =
                Self::handle_terminal_io(task_terminal, input_receiver, event_sender, task_id).await
            {
                println!(
                    "Error while handling terminal I/O: {} / {}",
                    e,
                    e.source().map(|e| e.to_string()).unwrap_or(String::new()),
                );
            }
        });

        Ok(pane)
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn update(&mut self, grid_update: &GridUpdate) {
        match grid_update {
            GridUpdate::AppendChar(c) => self.grid.update(*c),
            GridUpdate::NewLine => self.grid.new_line(),
        }
    }

    pub async fn process_input(&mut self, input: u8) {
        self.input_sender.send(input).await.unwrap();
    }

    async fn handle_terminal_io(
        mut terminal: Terminal,
        mut input_receiver: Receiver<u8>,
        event_sender: Sender<Event>,
        pane_id: PaneId,
    ) -> splix_error::Result<()> {
        loop {
            tokio::select! {
                Some(input) = input_receiver.recv() => Self::handle_terminal_input(&mut terminal, input).await?,
                Ok(chars) = terminal.read() => Self::handle_terminal_output(&chars, &event_sender, pane_id).await?,
            }
        }
    }

    async fn handle_terminal_input(terminal: &mut Terminal, input: u8) -> splix_error::Result<()> {
        terminal.write(input).await
    }

    async fn handle_terminal_output(
        chars: &[char],
        event_sender: &Sender<Event>,
        pane_id: PaneId,
    ) -> splix_error::Result<()> {
        for ch in chars.iter() {
            let update = if *ch == '\n' {
                GridUpdate::NewLine
            } else {
                GridUpdate::AppendChar(*ch)
            };

            event_sender
                .send(Event::PaneUpdate(PaneUpdateEvent::new(pane_id, update)))
                .await
                .map_err(|_| splix_error::Error::SendPaneUpdate)?;
        }

        Ok(())
    }
}
