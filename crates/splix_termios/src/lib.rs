mod alternate_screen;
mod raw_mode;

use alternate_screen::AlternateScreen;
use raw_mode::RawMode;

pub struct Termios {
    _raw_mode: RawMode,
    _alternate_screen: AlternateScreen,
}

impl Termios {
    pub fn new() -> splix_error::Result<Self> {
        let raw_mode = RawMode::new()?;
        let alternate_screen = AlternateScreen::new()?;

        Ok(Self {
            _raw_mode: raw_mode,
            _alternate_screen: alternate_screen,
        })
    }
}
