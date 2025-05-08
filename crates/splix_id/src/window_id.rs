use crate::SessionId;

#[derive(Clone, Copy)]
pub struct WindowId {
    session: SessionId,
    id: usize,
}

impl WindowId {
    pub fn new(session: SessionId, id: usize) -> Self {
        Self { session, id }
    }

    pub fn get(&self) -> usize {
        self.id
    }

    pub fn get_session(&self) -> SessionId {
        self.session
    }
}
