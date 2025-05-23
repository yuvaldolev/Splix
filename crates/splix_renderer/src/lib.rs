use std::io::{self, Write}; // Removed Stdout

use glam::UVec2;
use splix_pane::Pane;
use splix_window::Window;

pub struct Renderer<W: Write> { // Made generic
    screen_dimensions: UVec2,
    render_buffer: Vec<char>,
    previous_render_buffer: Vec<char>,
    stdout: W, // Use generic type
}

impl<W: Write> Renderer<W> { // Made generic
    pub fn new(screen_dimensions: UVec2, stdout: W) -> Self { // stdout passed as argument
        let buffer_size = (screen_dimensions.y * screen_dimensions.x) as usize;
        Self {
            screen_dimensions,
            render_buffer: vec![' '; buffer_size],
            previous_render_buffer: vec![' '; buffer_size],
            stdout, // Use passed argument
        }
    }

    pub fn begin_frame(&mut self) {
        self.set_cursor_home();

        // Copy render_buffer to previous_render_buffer
        self.previous_render_buffer.copy_from_slice(&self.render_buffer);

        // TODO: Should be probably removed later.
        self.reset_render_buffer();
    }

    pub fn end_frame(&mut self) {
        self.flush();
        self.stdout.flush().unwrap();
    }

    pub fn draw_window(&mut self, window: &Window) {
        self.draw_pane(window.get_pane(0));
    }

    fn render_buffer_index_from_position(&self, position: UVec2) -> usize {
        ((self.screen_dimensions.x * position.y) + position.x) as usize
    }

    fn reset_render_buffer(&mut self) {
        for y in 0..self.screen_dimensions.y {
            for x in 0..self.screen_dimensions.x {
                let index = self.render_buffer_index_from_position(UVec2::new(x, y));
                self.render_buffer[index] = ' ';
            }
        }
    }

    fn move_cursor_to(&mut self, x: u32, y: u32) {
        // ANSI escape codes are 1-indexed
        write!(self.stdout, "\x1B[{};{}H", y + 1, x + 1).unwrap();
    }

    fn set_cursor_home(&mut self) {
        // Using \x1B[H directly as it's simpler than move_cursor_to(0,0)
        // and ensures the specific "home" sequence.
        self.stdout.write_all(b"\x1B[H").unwrap();
    }

    fn flush(&mut self) {
        for y in 0..self.screen_dimensions.y {
            let mut current_x: u32 = 0;
            while current_x < self.screen_dimensions.x {
                let current_char_idx = self.render_buffer_index_from_position(UVec2::new(current_x, y));
                // Note: previous_render_buffer should be indexed the same way
                let prev_char_idx = current_char_idx; 

                if self.render_buffer[current_char_idx] != self.previous_render_buffer[prev_char_idx] {
                    // Start of a changed sequence
                    let mut sequence = String::new();
                    let sequence_start_x = current_x;

                    // Collect all consecutive changed characters
                    while current_x < self.screen_dimensions.x {
                        let char_idx = self.render_buffer_index_from_position(UVec2::new(current_x, y));
                        // Ensure we use the same index for previous_render_buffer for comparison
                        if self.render_buffer[char_idx] != self.previous_render_buffer[char_idx] {
                            sequence.push(self.render_buffer[char_idx]);
                            current_x += 1;
                        } else {
                            break; // Sequence of changes ended
                        }
                    }

                    // Move cursor to the start of the sequence and print it
                    self.move_cursor_to(sequence_start_x, y);
                    write!(self.stdout, "{}", sequence).unwrap();
                } else {
                    current_x += 1; // No change, move to next character
                }
            }
        }
        // self.stdout.flush().unwrap(); // This is called in end_frame()
    }

    fn draw_pane(&mut self, pane: &Pane) {
        for (y, line) in pane.get_grid().get_data().iter().enumerate() {
            if (y as u32) >= self.screen_dimensions.y {
                break;
            }

            for (x, character) in line.iter().enumerate() {
                if (x as u32) < self.screen_dimensions.x { // Check horizontal boundary
                    let index = self.render_buffer_index_from_position(UVec2::new(x as u32, y as u32));
                    self.render_buffer[index] = *character;
                } else {
                    // Optional: if a line from the pane is longer than the screen width,
                    // break from this inner loop to avoid unnecessary iteration.
                    break; 
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor; // To wrap Vec<u8> if we need Seek, but Vec<u8> is Write directly.
    use glam::UVec2;
    use splix_pane::{Grid, Pane};
    use splix_window::Window;
    use splix_id::{SessionId, WindowId, PaneId};

    // Helper function to create a Renderer with a Vec<u8> as stdout
    fn setup_renderer(width: u32, height: u32) -> Renderer<Vec<u8>> {
        Renderer::new(UVec2::new(width, height), Vec::new())
    }

    // Helper function to create a Window with a single Pane containing the given lines
    // For simplicity, using default IDs. Tests requiring specific ID interactions would need more setup.
    fn setup_window_with_pane_data(lines: Vec<Vec<char>>) -> Window {
        let session_id = SessionId::new(0);
        let window_id = WindowId::new(0);
        let pane_id = PaneId::new(0);
        let grid = Grid::new(lines);
        let pane = Pane::new(grid, window_id, pane_id);
        Window::new(vec![pane], session_id, window_id)
    }

    #[test]
    fn test_initial_render() {
        let mut renderer = setup_renderer(5, 1);
        let window = setup_window_with_pane_data(vec![vec!['H', 'e', 'l', 'l', 'o']]);

        renderer.begin_frame();
        renderer.draw_window(&window);
        renderer.end_frame();

        let output = String::from_utf8(renderer.stdout).unwrap();
        // Expected: set_cursor_home from begin_frame, then move_cursor_to(0,0) and print "Hello" from flush.
        // previous_render_buffer is all ' ', render_buffer gets "Hello".
        // So, "Hello" will be identified as the sequence to print.
        assert_eq!(output, "[H[1;1HHello");
    }

    #[test]
    fn test_diff_rendering_no_change() {
        let mut renderer = setup_renderer(5, 1);
        let window = setup_window_with_pane_data(vec![vec!['H', 'e', 'l', 'l', 'o']]);

        // Frame 1
        renderer.begin_frame();
        renderer.draw_window(&window);
        renderer.end_frame();

        renderer.stdout.clear(); // Clear stdout buffer

        // Frame 2 (no data change)
        renderer.begin_frame();
        renderer.draw_window(&window);
        renderer.end_frame();

        let output = String::from_utf8(renderer.stdout).unwrap();
        assert_eq!(output, "[H"); // Only set_cursor_home
    }

    #[test]
    fn test_diff_rendering_single_char_change() {
        let mut renderer = setup_renderer(5, 1);
        // Use a mutable window/pane setup or recreate for modification
        let mut lines = vec![vec!['H', 'e', 'l', 'l', 'o']];
        let window1 = setup_window_with_pane_data(lines.clone());

        // Frame 1
        renderer.begin_frame();
        renderer.draw_window(&window1);
        renderer.end_frame();

        renderer.stdout.clear();

        // Modify Pane data: "Hello" -> "Hollo" (char at index 1 changes 'e' -> 'o')
        lines[0][1] = 'o';
        let window2 = setup_window_with_pane_data(lines);

        // Frame 2
        renderer.begin_frame();
        renderer.draw_window(&window2);
        renderer.end_frame();
        
        let output = String::from_utf8(renderer.stdout).unwrap();
        assert_eq!(output, "[H[1;2Ho");
    }

    #[test]
    fn test_diff_rendering_multiple_consecutive_chars_change() {
        let mut renderer = setup_renderer(5, 1);
        let mut lines = vec![vec!['H', 'e', 'l', 'l', 'o']];
        let window1 = setup_window_with_pane_data(lines.clone());

        // Frame 1
        renderer.begin_frame();
        renderer.draw_window(&window1);
        renderer.end_frame();

        renderer.stdout.clear();

        // Modify Pane data: "Hello" -> "Hiyao" (ell -> iya)
        lines[0][1] = 'i';
        lines[0][2] = 'y';
        lines[0][3] = 'a';
        let window2 = setup_window_with_pane_data(lines);
        
        // Frame 2
        renderer.begin_frame();
        renderer.draw_window(&window2);
        renderer.end_frame();

        let output = String::from_utf8(renderer.stdout).unwrap();
        assert_eq!(output, "[H[1;2Hiya");
    }

    #[test]
    fn test_draw_pane_clipping_horizontal() {
        let mut renderer = setup_renderer(3, 1); // Screen width is 3
        let window = setup_window_with_pane_data(vec![vec!['H', 'e', 'l', 'l', 'o']]);

        renderer.begin_frame();
        renderer.draw_window(&window);
        renderer.end_frame();

        let output = String::from_utf8(renderer.stdout).unwrap();
        assert_eq!(output, "[H[1;1HHel");
    }

    #[test]
    fn test_draw_pane_clipping_vertical() {
        let mut renderer = setup_renderer(5, 1); // Screen height is 1
        let window = setup_window_with_pane_data(vec![
            vec!['H', 'e', 'l', 'l', 'o'],
            vec!['W', 'o', 'r', 'l', 'd']
        ]);

        renderer.begin_frame();
        renderer.draw_window(&window);
        renderer.end_frame();

        let output = String::from_utf8(renderer.stdout).unwrap();
        assert_eq!(output, "[H[1;1HHello"); // Only the first line should be rendered
    }
}
