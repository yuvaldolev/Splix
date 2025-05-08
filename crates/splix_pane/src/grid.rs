#[derive(Debug)]
pub enum GridUpdate {
    AppendChar(char),
    NewLine,
}

#[derive(Clone)]
pub struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn update(&mut self, c: char) {
        // For now, just append to the last line or create a new line
        if self.data.is_empty() {
            self.data.push(Vec::new());
        }
        self.data.last_mut().unwrap().push(c);
    }

    pub fn new_line(&mut self) {
        self.data.push(Vec::new());
    }

    pub fn get_data(&self) -> &[Vec<char>] {
        &self.data
    }
}
