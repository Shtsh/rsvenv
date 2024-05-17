pub mod local;
pub mod pyenv;
mod python;
pub mod rsenv;
pub mod traits;
mod utils;

use anyhow::Context;
use anyhow::Result;
use simplelog::info;

use std::collections::HashSet;
use std::io;
use std::io::Write;

use self::local::Local;
use self::pyenv::Pyenv;
use self::rsenv::Rsenv;
use self::traits::VirtualEnvCompatible;
use crate::shell::SupportedShell;
use crate::virtualenv::utils::{get_current_dir, is_virtualenv};

pub struct VirtualEnvironment<'a> {
    // Venv path
    pub kind: &'a dyn VirtualEnvCompatible,
    pub shell: SupportedShell,
}

impl<'a> VirtualEnvironment<'a> {
    pub fn new(kind: &'a dyn VirtualEnvCompatible) -> Result<Self> {
        Ok(VirtualEnvironment {
            kind,
            shell: SupportedShell::new()?,
        })
    }

    pub fn list(&self) -> HashSet<String> {
        self.kind.list()
    }

    pub fn detect() -> Option<Self> {
        if Rsenv.relevant() {
            return Self::new(&Rsenv).ok();
        }

        if Pyenv.relevant() {
            return Self::new(&Pyenv).ok();
        }

        if Local.relevant() {
            return Self::new(&Local).ok();
        }

        None
    }

    pub fn activate(&self, venv_name: Option<&String>) -> Result<()> {
        let path = self.kind.path(venv_name)?;
        is_virtualenv(&path)?;
        info!("Activating {path:?}");
        let command = self.shell.render_activate(path, get_current_dir()?)?;
        io::stdout()
            .write_all(command.as_bytes())
            .context("Unable to write to STDOUT")?;

        Ok(())
    }

    pub fn deactivate(force: bool) -> Result<()> {
        let value = std::env::var("VIRTUAL_ENV").context("Unable to read VIRTUAL_ENV")?;
        info!("Deactivating {value:?}");
        let shell = SupportedShell::new()?;

        let command = shell.render_deactivate(force)?;
        io::stdout()
            .write_all(command.as_bytes())
            .context("Unable to write to STDOUT")?;

        Ok(())
    }
}
