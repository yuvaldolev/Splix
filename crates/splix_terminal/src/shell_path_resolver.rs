use std::{env, path::PathBuf};

pub struct ShellPathResolver;

const SHELL_ENVIRONMENT_VARIABLE: &str = "SHELL";

impl ShellPathResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve(&self) -> PathBuf {
        // TODO: Handle cases where the `SHELL` environment variable doesn't exist.
        PathBuf::from(env::var(SHELL_ENVIRONMENT_VARIABLE).unwrap())
    }
}
