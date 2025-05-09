use glam::UVec2;
use splix_renderer::Renderer;
use terminal_size::{Height, Width};
use tokio::sync::mpsc::{self, Receiver, Sender};

use splix_event::{Event, PaneUpdateEvent};
use splix_id::SessionId;
use splix_session::Session;
use splix_termios::Termios;

pub struct Splix {
    _termios: Termios,
    sessions: Vec<Session>,
    next_session_id: usize,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    renderer: Renderer,
}

const EVENT_CHANNEL_CAPACITY: usize = 1024;

impl Splix {
    pub fn new() -> splix_error::Result<Self> {
        let screen_dimensions = Self::retrieve_screen_dimensions()?;

        let termios = Termios::new()?;
        let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) =
            mpsc::channel(EVENT_CHANNEL_CAPACITY);

        let mut splix = Self {
            _termios: termios,
            sessions: Vec::new(),
            next_session_id: 0,
            event_sender,
            event_receiver,
            renderer: Renderer::new(screen_dimensions),
        };

        splix.new_session()?;

        Ok(splix)
    }

    pub async fn run(&mut self) -> splix_error::Result<()> {
        while let Some(event) = self.event_receiver.recv().await {
            self.handle_event(&event);
        }

        Ok(())
    }

    fn retrieve_screen_dimensions() -> splix_error::Result<UVec2> {
        if let Some((Width(terminal_width), Height(terminal_height))) =
            terminal_size::terminal_size()
        {
            Ok(UVec2::new(terminal_width as u32, terminal_height as u32))
        } else {
            Err(splix_error::Error::RetrieveTerminalSize)
        }
    }

    fn new_session(&mut self) -> splix_error::Result<()> {
        let id = SessionId::new(self.next_session_id);
        let session = Session::new(id, self.event_sender.clone())?;
        self.sessions.push(session);
        self.next_session_id += 1;

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::PaneUpdate(event) => self.handle_pane_update(event),
        }
    }

    fn handle_pane_update(&mut self, event: &PaneUpdateEvent) {
        let session = &mut self.sessions[event.get_pane().get_window().get_session().get()];
        session.update_pane(event.get_pane(), event.get_grid_update());
        self.redraw();
    }

    fn redraw(&mut self) {
        self.renderer.begin_frame();
        self.renderer.draw_window(self.sessions[0].get_window(0));
        self.renderer.end_frame();
    }
}
