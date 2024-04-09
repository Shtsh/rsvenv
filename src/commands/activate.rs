use clap::Parser;
use error_stack::{Report, Result};
use simplelog::{debug, error};

use crate::{
    errors::{CommandExecutionError, VirtualEnvError},
    virtualenv::{pyenv::Pyenv, rsenv::Rsenv, traits::VirtualEnvCompatible, VirtualEnvironment},
};

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(help = "string matching a Python version known to pyenv-rs")]
    virtualenv: String,
}

fn try_activate(f: &dyn VirtualEnvCompatible, venv: &String) -> Result<(), VirtualEnvError> {
    if f.list().contains(venv) {
        let venv_struct = VirtualEnvironment { kind: f };
        if let Err(e) = venv_struct.activate(Some(venv)) {
            error!("{e}");
            return Err(e);
        };
    }
    Err(Report::new(VirtualEnvError::NotVirtualEnv(
        venv.to_string(),
    )))
}

impl Command {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        if let Err(e) = VirtualEnvironment::deactivate(true) {
            debug!("{e}");
        };
        if let Ok(_) = try_activate(&Rsenv, &self.virtualenv) {
            return Ok(());
        }
        if let Ok(_) = try_activate(&Pyenv, &self.virtualenv) {
            return Ok(());
        }
        error!(
            "Failed to find and activate virtual environment {}",
            self.virtualenv
        );
        Err(Report::new(CommandExecutionError {
            command: "activate".into(),
        })
        .attach_printable(format!("Unable to activate vitualenv {}", self.virtualenv)))
    }
}
