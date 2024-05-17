use crate::virtualenv::VirtualEnvironment;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(long, short, action)]
    auto: bool,
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        VirtualEnvironment::deactivate(true)
    }
}
