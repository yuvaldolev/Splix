use splix_error::Result;
use splix_pane::{Grid, GridUpdate, Pane, PaneUpdate};
use tokio::sync::mpsc::{channel, Receiver};

pub struct Window {
    panes: Vec<Pane>,
    update_receiver: Receiver<PaneUpdate>,
}

impl Window {
    pub fn new() -> Result<Self> {
        let (update_sender, update_receiver) = channel(32);
        let pane = Pane::new(0, update_sender)?;

        Ok(Self {
            panes: vec![pane],
            update_receiver,
        })
    }

    pub fn get_pane_grid(&mut self, pane_id: usize) -> Option<&Grid> {
        // Update the grid with any pending updates for this pane
        while let Ok(update) = self.update_receiver.try_recv() {
            if update.pane_id == pane_id {
                match update.update {
                    GridUpdate::AppendChar(c) => self.panes[pane_id].grid.update(c),
                    GridUpdate::NewLine => self.panes[pane_id].grid.new_line(),
                }
            }
        }
        self.panes.get(pane_id).map(|pane| pane.get_grid())
    }

    pub fn add_pane(&mut self) -> Result<()> {
        let pane_id = self.panes.len();
        let (update_sender, _) = channel(32);
        let pane = Pane::new(pane_id, update_sender)?;
        self.panes.push(pane);
        Ok(())
    }
}
