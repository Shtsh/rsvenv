use crate::virtualenv::Rsenv;
use clap::Parser;
use simplelog::error;

#[derive(Debug, Parser)]
pub struct DeleteCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

impl DeleteCommand {
    pub fn execute(&self) {
        if let Err(err) = Rsenv.delete(&self.venv) {
            error!("{}", err);
        }
    }
}
