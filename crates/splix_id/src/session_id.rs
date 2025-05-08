#[derive(Clone, Copy)]
pub struct SessionId {
    id: usize,
}

impl SessionId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}
