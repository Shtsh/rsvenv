use crate::virtualenv::{Pyenv, Rsenv, VirtualEnvCompatible};
use clap::Parser;
use std::collections::HashSet;
use std::io;
use std::io::Write;

#[derive(Debug, Parser)]
pub struct ListCommand {}

fn print_venvs(rsenv_venvs: HashSet<String>) {
    for name in rsenv_venvs {
        io::stdout().write_all("\t".as_bytes()).unwrap_or(());
        io::stdout().write_all(name.as_bytes()).unwrap_or(());
        io::stdout().write_all("\n".as_bytes()).unwrap_or(());
    }
}

impl ListCommand {
    pub fn execute(&self) {
        let rsenv_venvs = Rsenv.list();
        if !rsenv_venvs.is_empty() {
            io::stdout()
                .write_all("Rsenv environments:\n".as_bytes())
                .unwrap_or(());
            print_venvs(rsenv_venvs);
        }
        let pyenv_venvs = Pyenv.list();
        if !pyenv_venvs.is_empty() {
            io::stdout()
                .write_all("Pyenv environments:\n".as_bytes())
                .unwrap_or(());
            print_venvs(pyenv_venvs);
        }
    }
}
