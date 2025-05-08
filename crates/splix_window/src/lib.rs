use splix_error::Result;
use splix_id::{PaneId, WindowId};
use splix_pane::{Grid, GridUpdate, Pane, PaneUpdate};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct Window {
    id: WindowId,
    panes: Vec<Pane>,
    next_pane_id: usize,
    update_sender: Sender<PaneUpdate>,
    update_receiver: Receiver<PaneUpdate>,
}

impl Window {
    pub fn new(id: WindowId) -> Result<Self> {
        let (update_sender, update_receiver) = mpsc::channel(32);

        let mut window = Self {
            id,
            panes: Vec::new(),
            next_pane_id: 0,
            update_sender,
            update_receiver,
        };

        window.new_pane()?;

        Ok(window)
    }

    pub fn get_pane_grid(&mut self, pane_id: usize) -> Option<&Grid> {
        // Update the grid with any pending updates for this pane
        while let Ok(update) = self.update_receiver.try_recv() {
            if update.pane_id.get() == pane_id {
                match update.update {
                    GridUpdate::AppendChar(c) => {
                        println!("update: {}", c);
                        self.panes[pane_id].grid.update(c);
                    }
                    GridUpdate::NewLine => {
                        println!("new line");
                        self.panes[pane_id].grid.new_line();
                    }
                }
            }
        }
        self.panes.get(pane_id).map(|pane| pane.get_grid())
    }

    fn new_pane(&mut self) -> splix_error::Result<()> {
        let id = PaneId::new(self.id, self.next_pane_id);
        let pane = Pane::new(id, self.update_sender.clone())?;
        self.panes.push(pane);
        self.next_pane_id += 1;

        Ok(())
    }
}
