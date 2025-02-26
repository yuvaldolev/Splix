pub struct AnsiEncoder;

const ANSI_ESCAPE_CODE_PREFIX: &str = "\x1B[";

impl AnsiEncoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, escape_code: &str) -> String {
        format!("{ANSI_ESCAPE_CODE_PREFIX}{escape_code}")
    }
}
