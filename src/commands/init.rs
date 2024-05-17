use crate::shell::SupportedShell;
use anyhow::{Context, Result};
use clap::Parser;
use std::io;
use std::io::Write;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn execute(&self) -> Result<()> {
        let shell = SupportedShell::new().context("Unable to detect current shell")?;

        let hook = shell.get_hook();

        io::stdout()
            .write_all(hook.as_bytes())
            .context("Unable to write hook to STDOUT!")?;
        Ok(())
    }
}
