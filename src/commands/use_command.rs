use crate::virtualenv::{Pyenv, Rsenv, VirtualEnvCompatible};
use clap::Parser;
use simplelog::error;
use std::error::Error;

#[derive(Debug, Parser)]
pub struct UseCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

fn try_save(f: &dyn VirtualEnvCompatible, venv: &String) -> Result<(), Box<dyn Error>> {
    if f.list().contains(venv) {
        if let Err(e) = f.save(venv) {
            error!("Error saving virtual env: {e}");
            return Err(e);
        }
        return Ok(());
    }
    Err("No such venv".into())
}

impl UseCommand {
    pub fn execute(&self) {
        if try_save(&Rsenv, &self.venv).is_ok() {
            return;
        };
        if try_save(&Pyenv, &self.venv).is_ok() {
            return;
        };
        error!("Virtual environment {} doesn't exist", &self.venv);
    }
}
