use super::utils::is_virtualenv;
use anyhow::Result;
use std::{collections::HashSet, path::PathBuf};

pub trait VirtualEnvCompatible {
    fn root_dir(&self) -> Result<PathBuf>;

    fn list(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn relevant(&self) -> bool;

    fn venv_name(&self) -> Result<String>;

    fn path(&self, name: Option<&String>) -> Result<PathBuf> {
        let venv_name = self.venv_name()?;
        let b = self.root_dir()?.join(name.unwrap_or(&venv_name));
        is_virtualenv(&b)?;
        Ok(b)
    }

    fn save(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}
