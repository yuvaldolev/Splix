use std::io::{self, Stdout, Write};

use glam::UVec2;
use splix_pane::Pane;
use splix_window::Window;

pub struct Renderer {
    screen_dimensions: UVec2,
    render_buffer: Vec<char>,
    stdout: Stdout,
}

impl Renderer {
    pub fn new(screen_dimensions: UVec2) -> Self {
        Self {
            screen_dimensions,
            render_buffer: vec![' '; (screen_dimensions.y * screen_dimensions.x) as usize],
            stdout: io::stdout(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.reset_cursor();

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

    fn reset_cursor(&mut self) {
        self.stdout.write_all(b"\x1B[H").unwrap();
    }

    fn flush(&mut self) {
        for y in 0..self.screen_dimensions.y {
            for x in 0..self.screen_dimensions.x {
                let character =
                    self.render_buffer[self.render_buffer_index_from_position(UVec2::new(x, y))];
                write!(self.stdout, "{character}").unwrap();
            }

            self.move_cursor_to_next_line();
        }
    }

    fn move_cursor_to_next_line(&mut self) {
        self.stdout.write_all(b"\x1B[1E").unwrap();
    }

    fn draw_pane(&mut self, pane: &Pane) {
        for (y, line) in pane.get_grid().get_data().iter().enumerate() {
            if (y as u32) >= self.screen_dimensions.y {
                break;
            }

            for (x, character) in line.iter().enumerate() {
                let index = self.render_buffer_index_from_position(UVec2::new(x as u32, y as u32));
                self.render_buffer[index] = *character;
            }
        }
    }
}
