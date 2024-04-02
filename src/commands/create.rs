use crate::virtualenv::Rsenv;
use clap::Parser;
use simplelog::error;

#[derive(Debug, Parser)]
pub struct CreateCommand {
    #[clap(help = "Path to python executable")]
    python: String,
    #[clap(help = "Virtualenv name")]
    name: String,
}

impl CreateCommand {
    pub fn execute(&self) {
        if let Err(err) = Rsenv.create(&self.name, &self.python) {
            error!("{}", err);
        }
    }
}
