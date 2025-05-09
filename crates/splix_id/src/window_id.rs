use crate::SessionId;

#[derive(Clone, Copy, Debug)]
pub struct WindowId {
    id: usize,
    session: SessionId,
}

impl WindowId {
    pub fn new(id: usize, session: SessionId) -> Self {
        Self { id, session }
    }

    pub fn get(&self) -> usize {
        self.id
    }

    pub fn get_session(&self) -> SessionId {
        self.session
    }
}
