use splix_id::{SessionId, WindowId};
use splix_window::Window;

pub struct Session {
    id: SessionId,
    windows: Vec<Window>,
    next_window_id: usize,
}

impl Session {
    pub fn new(id: SessionId) -> splix_error::Result<Self> {
        let mut session = Self {
            id,
            windows: Vec::new(),
            next_window_id: 0,
        };

        session.new_window()?;

        Ok(session)
    }

    pub async fn attach(&mut self) {
        loop {
            let grid = self.windows[0].get_pane_grid(0).unwrap();
            for line in grid.get_data().iter() {
                for char in line.iter() {
                    print!("{}", char);
                }
                println!();
            }
        }
        // loop {
        // let byte = self.terminal.read().await;
        // io::stdout().write_u8(byte).unwrap();
        // }
    }

    fn new_window(&mut self) -> splix_error::Result<()> {
        let id = WindowId::new(self.id, self.next_window_id);
        let window = Window::new(id)?;
        self.windows.push(window);
        self.next_window_id += 1;

        Ok(())
    }
}
