use crate::WindowId;

#[derive(Clone, Copy)]
pub struct PaneId {
    window: WindowId,
    id: usize,
}

impl PaneId {
    pub fn new(window: WindowId, id: usize) -> Self {
        Self { window, id }
    }

    pub fn get(&self) -> usize {
        self.id
    }

    pub fn get_window(&self) -> WindowId {
        self.window
    }
}
