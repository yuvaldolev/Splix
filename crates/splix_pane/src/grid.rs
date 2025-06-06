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

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn new_grid_is_empty() {
        let grid = Grid::new();
        assert!(grid.get_data().is_empty());
    }

    #[test]
    fn update_appends_chars() {
        let mut grid = Grid::new();
        grid.update('a');
        grid.update('b');
        assert_eq!(grid.get_data(), &vec![vec!['a', 'b']]);
    }

    #[test]
    fn new_line_creates_new_empty_line() {
        let mut grid = Grid::new();
        grid.update('a');
        grid.new_line();
        grid.update('b');
        assert_eq!(grid.get_data(), &vec![vec!['a'], vec!['b']]);
    }
}
