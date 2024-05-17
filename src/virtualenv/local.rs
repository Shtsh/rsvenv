use anyhow::{bail, Result};
use std::path::PathBuf;

use super::{
    traits::VirtualEnvCompatible,
    utils::{get_current_dir, is_virtualenv},
};

#[derive(Debug)]
pub struct Local;

impl VirtualEnvCompatible for Local {
    fn root_dir(&self) -> Result<PathBuf> {
        get_current_dir()
    }

    fn relevant(&self) -> bool {
        self.venv_name().is_ok()
    }

    fn venv_name(&self) -> Result<String> {
        let current_path = self.root_dir()?;
        for local_venv_path in ["venv", ".venv", "virtualenv", ".virtualenv"] {
            if is_virtualenv(&current_path.join(local_venv_path)).is_ok() {
                return Ok(local_venv_path.to_string());
            }
        }
        bail!("No local venv in {}", current_path.as_path().display());
    }
}
