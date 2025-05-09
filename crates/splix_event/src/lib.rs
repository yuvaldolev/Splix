mod pane_update_event;

pub use pane_update_event::{GridUpdate, PaneUpdateEvent};

#[derive(Debug)]
pub enum Event {
    PaneUpdate(PaneUpdateEvent),
}
