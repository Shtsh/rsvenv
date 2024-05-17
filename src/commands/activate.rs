use anyhow::{bail, Result};
use clap::Parser;
use simplelog::debug;

use crate::virtualenv::{pyenv::Pyenv, rsenv::Rsenv, VirtualEnvironment};

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(help = "string matching a Python version known to pyenv-rs")]
    virtualenv: String,
}

fn try_activate(v: VirtualEnvironment, venv: &String) -> Result<()> {
    if v.list().contains(venv) {
        if let Err(e) = v.activate(Some(venv)) {
            debug!("Unable to activate venv: {e:?}");
            bail!(e);
        };
        return Ok(());
    }
    bail!("{venv} is not a virtual environment")
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        VirtualEnvironment::deactivate(true)?;
        if try_activate(VirtualEnvironment::new(&Rsenv)?, &self.virtualenv).is_ok() {
            return Ok(());
        }
        if try_activate(VirtualEnvironment::new(&Pyenv)?, &self.virtualenv).is_ok() {
            return Ok(());
        }
        bail!(
            "Failed to find and activate virtual environment {}",
            self.virtualenv
        );
    }
}
