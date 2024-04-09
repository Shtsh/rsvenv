use crate::{
    errors::{CommandExecutionError, VirtualEnvError},
    virtualenv::{pyenv::Pyenv, rsenv::Rsenv, traits::VirtualEnvCompatible},
};
use clap::Parser;
use error_stack::{Report, Result};
use simplelog::error;

#[derive(Debug, Parser)]
pub struct UseCommand {
    #[clap(help = "Virtual environment name")]
    venv: String,
}

fn try_save(f: &dyn VirtualEnvCompatible, venv: String) -> Result<(), VirtualEnvError> {
    if f.list().contains(&venv) {
        f.save(&venv)?;
        return Ok(());
    }
    Err(
        Report::new(VirtualEnvError::NotVirtualEnv(venv.clone())).attach_printable(format!(
            "{} not in the list of existing virtual environments",
            venv
        )),
    )
}

impl UseCommand {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        match try_save(&Rsenv, self.venv.clone()) {
            Ok(()) => return Ok(()),
            Err(e) => error!("{e}"),
        }
        match try_save(&Pyenv, self.venv.clone()) {
            Ok(()) => return Ok(()),
            Err(e) => error!("{e}"),
        }

        error!("Virtual environment {} doesn't exist", &self.venv);
        Ok(())
    }
}
