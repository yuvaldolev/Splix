use crate::WindowId;

#[derive(Clone, Copy, Debug)]
pub struct PaneId {
    id: usize,
    window: WindowId,
}

impl PaneId {
    pub fn new(id: usize, window: WindowId) -> Self {
        Self { id, window }
    }

    pub fn get(&self) -> usize {
        self.id
    }

    pub fn get_window(&self) -> WindowId {
        self.window
    }
}
