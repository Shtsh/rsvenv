use crate::{errors::CommandExecutionError, virtualenv::rsenv::Rsenv};
use clap::Parser;
use error_stack::{Result, ResultExt};

#[derive(Debug, Parser)]
pub struct DeleteCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

impl DeleteCommand {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        Rsenv
            .delete(self.venv.clone())
            .change_context(CommandExecutionError {
                command: "delete".into(),
            })
    }
}
