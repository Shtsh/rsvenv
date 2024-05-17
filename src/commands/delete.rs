use crate::virtualenv::rsenv::Rsenv;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DeleteCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

impl DeleteCommand {
    pub fn execute(&self) -> Result<()> {
        Rsenv.delete(self.venv.clone())
    }
}
