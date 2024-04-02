use crate::virtualenv::VirtualEnvironment;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(long, short, action)]
    auto: bool,
}

impl Command {
    pub fn execute(&self) {
        let _ = VirtualEnvironment::deactivate(true);
    }
}
