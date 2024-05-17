use anyhow::{Context, Result};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use simplelog::{debug, info};

use super::{
    traits::VirtualEnvCompatible,
    utils::{get_current_dir, get_venvs_by_glob},
};

#[derive(Debug)]
pub struct Pyenv;

impl VirtualEnvCompatible for Pyenv {
    fn root_dir(&self) -> Result<PathBuf> {
        let root = std::env::var("PYENV_ROOT").unwrap_or("~/.pyenv".to_string());
        let expanded = shellexpand::full(&root)
            .context("unable to expand PYENV_ROOT to the actual path")?
            .to_string();
        Ok(Path::new(&expanded).to_path_buf().join("versions"))
    }

    fn list(&self) -> HashSet<String> {
        if let Ok(root) = self.root_dir() {
            return get_venvs_by_glob("*/envs/*".into(), &root).unwrap_or_default();
        }
        HashSet::new()
    }

    fn relevant(&self) -> bool {
        let current_dir = get_current_dir();
        if current_dir.is_err() {
            return false;
        };

        if let Ok(metadata) = fs::symlink_metadata(current_dir.unwrap().join(".python-version")) {
            if metadata.is_file() {
                debug!("Found .python-version");
                return true;
            }
        }
        false
    }

    fn venv_name(&self) -> Result<String> {
        Ok(
            fs::read_to_string(get_current_dir()?.join(".python-version"))
                .context("Unable to read .python-verion")?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<()> {
        File::create(".python-version")
            .context("Unable to create .python-version")?
            .write_all(name.as_bytes())
            .context("Unable to save data .python-version")?;
        info!("Saved changes to .python-version");
        fs::remove_file(".python-virtualenv").unwrap_or_default();
        Ok(())
    }
}
