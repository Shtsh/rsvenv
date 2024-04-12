use clap::Parser;
use error_stack::{Report, Result};
use simplelog::{debug, error};

use crate::{
    errors::{CommandExecutionError, VirtualEnvError},
    virtualenv::{pyenv::Pyenv, rsenv::Rsenv, VirtualEnvironment},
};

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(help = "string matching a Python version known to pyenv-rs")]
    virtualenv: String,
}

fn try_activate(v: VirtualEnvironment, venv: &String) -> Result<(), VirtualEnvError> {
    if v.kind.list().contains(venv) {
        if let Err(e) = v.activate(Some(venv)) {
            error!("{e}");
            return Err(e);
        };
        return Ok(());
    }
    Err(
        Report::new(VirtualEnvError::NotVirtualEnv(venv.to_string()))
            .attach_printable("{venv} is not a virtual environment"),
    )
}

impl Command {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        if let Err(e) = VirtualEnvironment::deactivate(true) {
            debug!("{e}");
        };
        if try_activate(VirtualEnvironment { kind: &Rsenv }, &self.virtualenv).is_ok() {
            return Ok(());
        }
        if try_activate(VirtualEnvironment { kind: &Pyenv }, &self.virtualenv).is_ok() {
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
