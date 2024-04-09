use crate::{errors::CommandExecutionError, virtualenv::VirtualEnvironment};
use clap::Parser;
use error_stack::{Result, ResultExt};

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(long, short, action)]
    auto: bool,
}

impl Command {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        VirtualEnvironment::deactivate(true).change_context(CommandExecutionError {
            command: "deactivate".into(),
        })
    }
}
