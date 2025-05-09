use tokio::sync::mpsc::Sender;

use splix_event::Event;
use splix_id::{SessionId, WindowId};
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

    fn new_window(&mut self) -> splix_error::Result<()> {
        let id = WindowId::new(self.next_window_id, self.id);
        let window = Window::new(id, self.event_sender.clone())?;
        self.windows.push(window);
        self.next_window_id += 1;

        Ok(())
    }
}
