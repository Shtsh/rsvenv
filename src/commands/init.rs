use crate::errors::CommandExecutionError;
use crate::shell::SupportedShell;
use clap::Parser;
use error_stack::{Result, ResultExt};
use std::io;
use std::io::Write;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        let error_context = CommandExecutionError {
            command: "init".into(),
        };
        let shell = SupportedShell::new()
            .change_context(error_context.clone())
            .attach("Unable to detect current shell")?;

        let hook = shell.get_hook();

        io::stdout()
            .write_all(hook.as_bytes())
            .change_context(error_context)
            .attach("Unable to write hook to STDOUT!")?;
        Ok(())
    }
}
