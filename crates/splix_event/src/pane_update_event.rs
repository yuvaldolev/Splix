use splix_id::PaneId;

#[derive(Debug)]
pub struct PaneUpdateEvent {
    pane: PaneId,
}

impl PaneUpdateEvent {
    pub fn new(pane: PaneId) -> Self {
        Self { pane }
    }
}
