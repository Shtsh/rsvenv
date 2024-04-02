use crate::virtualenv::{Pyenv, Rsenv, VirtualEnvCompatible, VirtualEnvironment};
use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(help = "string matching a Python version known to pyenv-rs")]
    virtualenv: String,
}

fn try_activate(f: &dyn VirtualEnvCompatible, venv: &String) -> Result<(), Box<dyn Error>> {
    if f.list().contains(venv) {
        VirtualEnvironment { kind: f }.activate(Some(venv))?;
    }
    Err("No such venv".into())
}
impl Command {
    pub fn execute(&self) {
        VirtualEnvironment::deactivate(true).unwrap_or(());
        try_activate(&Rsenv, &self.virtualenv).unwrap_or(());
        try_activate(&Pyenv, &self.virtualenv).unwrap_or(());
    }
}
