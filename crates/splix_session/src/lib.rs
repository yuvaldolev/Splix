use tokio::sync::mpsc::Sender;

use splix_event::{Event, GridUpdate};
use splix_id::{PaneId, SessionId, WindowId};
use splix_window::Window;

pub struct Session {
    id: SessionId,
    event_sender: Sender<Event>,
    windows: Vec<Window>,
    next_window_id: usize,
}

impl Session {
    pub fn new(id: SessionId, event_sender: Sender<Event>) -> splix_error::Result<Self> {
        let mut session = Self {
            id,
            event_sender,
            windows: Vec::new(),
            next_window_id: 0,
        };

        session.new_window()?;

        Ok(session)
    }

    pub async fn attach(&mut self) {
        // loop {
        //     let grid = self.windows[0].get_pane_grid(0).unwrap();
        //     for line in grid.get_data().iter() {
        //         for char in line.iter() {
        //             print!("{}", char);
        //         }
        //         println!();
        //     }
        // }
        // loop {
        // let byte = self.terminal.read().await;
        // io::stdout().write_u8(byte).unwrap();
        // }
    }

    /// TODO: Should probably use `WindowId` instead of `usize`.
    pub fn get_window(&self, index: usize) -> &Window {
        &self.windows[index]
    }

    pub fn update_pane(&mut self, pane: PaneId, grid_update: &GridUpdate) {
        let window = &mut self.windows[pane.get_window().get()];
        window.update_pane(pane, grid_update);
    }

    fn new_window(&mut self) -> splix_error::Result<()> {
        let id = WindowId::new(self.next_window_id, self.id);
        let window = Window::new(id, self.event_sender.clone())?;
        self.windows.push(window);
        self.next_window_id += 1;

        Ok(())
    }
}
