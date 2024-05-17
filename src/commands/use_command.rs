use crate::virtualenv::{pyenv::Pyenv, rsenv::Rsenv, traits::VirtualEnvCompatible};
use anyhow::{bail, Result};
use clap::Parser;
use simplelog::debug;

#[derive(Debug, Parser)]
pub struct UseCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

fn try_save(f: &dyn VirtualEnvCompatible, venv: String) -> Result<()> {
    if f.list().contains(&venv) {
        f.save(&venv)?;
        return Ok(());
    }
    bail!("{} not in the list of existing virtual environments", venv)
}

impl UseCommand {
    pub fn execute(&self) -> Result<()> {
        match try_save(&Rsenv, self.venv.clone()) {
            Ok(()) => return Ok(()),
            Err(e) => debug!("Rsenv: {e}"),
        }
        match try_save(&Pyenv, self.venv.clone()) {
            Ok(()) => return Ok(()),
            Err(e) => debug!("Pyenv: {e}"),
        }

        bail!("Virtual environment {} doesn't exist", &self.venv);
    }
}
