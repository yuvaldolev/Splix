mod pane_update_event;

pub use pane_update_event::PaneUpdateEvent;

pub enum Event {
    PaneUpdate(PaneUpdateEvent),
}
