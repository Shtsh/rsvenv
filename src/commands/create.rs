use anyhow::Result;
use clap::Parser;

use crate::virtualenv::rsenv::Rsenv;

#[derive(Debug, Parser)]
pub struct CreateCommand {
    #[clap(help = "Path to python executable")]
    python: String,
    #[clap(help = "Virtualenv name")]
    name: String,
}

impl CreateCommand {
    pub fn execute(&self) -> Result<()> {
        Rsenv.create(&self.name, &self.python)
    }
}
