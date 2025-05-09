mod pane_update_event;

pub use pane_update_event::PaneUpdateEvent;

#[derive(Debug)]
pub enum Event {
    PaneUpdate(PaneUpdateEvent),
}
