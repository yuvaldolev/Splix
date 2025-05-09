use splix_id::PaneId;

#[derive(Debug)]
pub enum GridUpdate {
    AppendChar(char),
    NewLine,
}

#[derive(Debug)]
pub struct PaneUpdateEvent {
    pane: PaneId,
    grid_update: GridUpdate,
}

impl PaneUpdateEvent {
    pub fn new(pane: PaneId, grid_update: GridUpdate) -> Self {
        Self { pane, grid_update }
    }

    pub fn get_pane(&self) -> PaneId {
        self.pane
    }

    pub fn get_grid_update(&self) -> &GridUpdate {
        &self.grid_update
    }
}
