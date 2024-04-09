use clap::Parser;
use error_stack::{Result, ResultExt};

use crate::{errors::CommandExecutionError, virtualenv::rsenv::Rsenv};

#[derive(Debug, Parser)]
pub struct CreateCommand {
    #[clap(help = "Path to python executable")]
    python: String,
    #[clap(help = "Virtualenv name")]
    name: String,
}

impl CreateCommand {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        Rsenv
            .create(&self.name, &self.python)
            .change_context(CommandExecutionError {
                command: "create".into(),
            })?;
        Ok(())
    }
}
