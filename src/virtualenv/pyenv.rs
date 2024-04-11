use error_stack::{Result, ResultExt};
use std::{
    collections::HashSet,
    fs::{self, File},
    path::{Path, PathBuf},
};

use std::io::Write;

use simplelog::{debug, info};

use crate::errors::VirtualEnvError;

use super::{
    traits::VirtualEnvCompatible,
    utils::{get_current_dir, get_venvs_by_glob},
};

#[derive(Debug)]
pub struct Pyenv;

impl VirtualEnvCompatible for Pyenv {
    fn root_dir(&self) -> Result<PathBuf, VirtualEnvError> {
        let root = std::env::var("PYENV_ROOT").unwrap_or("~/.pyenv".to_string());
        let expanded = shellexpand::full(&root)
            .change_context(VirtualEnvError::ConfigurationError)
            .attach_printable("unable to expand PYENV_ROOT to the actual path")?
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

    fn venv_name(&self) -> Result<std::string::String, VirtualEnvError> {
        Ok(
            fs::read_to_string(get_current_dir()?.join(".python-version"))
                .change_context(VirtualEnvError::IOError)
                .attach_printable("Unable to read .python-verion")?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<(), VirtualEnvError> {
        File::create(".python-version")
            .change_context(VirtualEnvError::IOError)
            .attach_printable("Unable to create .python-version")?
            .write_all(name.as_bytes())
            .change_context(VirtualEnvError::IOError)
            .attach_printable("Unable to save data .python-version")?;
        info!("Saved changes to .python-version");
        fs::remove_file(".python-virtualenv").unwrap_or_default();
        Ok(())
    }
}
