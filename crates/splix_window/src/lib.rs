use tokio::sync::mpsc::Sender;

use splix_error::Result;
use splix_event::Event;
use splix_id::{PaneId, WindowId};
use splix_pane::Pane;

pub struct Window {
    id: WindowId,
    event_sender: Sender<Event>,
    panes: Vec<Pane>,
    next_pane_id: usize,
}

impl Window {
    pub fn new(id: WindowId, event_sender: Sender<Event>) -> Result<Self> {
        let mut window = Self {
            id,
            event_sender,
            panes: Vec::new(),
            next_pane_id: 0,
        };

        window.new_pane()?;

        Ok(window)
    }

    fn new_pane(&mut self) -> splix_error::Result<()> {
        let id = PaneId::new(self.next_pane_id, self.id);
        let pane = Pane::new(id, self.event_sender.clone())?;
        self.panes.push(pane);
        self.next_pane_id += 1;

        Ok(())
    }
}
