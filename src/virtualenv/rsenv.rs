use std::{
    collections::HashSet,
    fs::{self, File},
    path::{Path, PathBuf},
    process,
};

use crate::{configuration::SETTINGS, errors::VirtualEnvError};
use error_stack::{Report, Result, ResultExt};
use simplelog::{error, info};
use std::io::Write;

use super::{
    traits::VirtualEnvCompatible,
    utils::{get_current_dir, get_venvs_by_glob},
};

#[derive(Debug)]
pub struct Rsenv;

impl Rsenv {
    pub fn create(&self, name: &String, python: &String) -> Result<(), VirtualEnvError> {
        if self.list().contains(name) {
            return Err(Report::new(VirtualEnvError::AlreadyExists(
                name.to_string(),
            )));
        }

        let root_dir = self.root_dir()?;

        let path = root_dir.as_path();
        if !path.exists() {
            let _ = fs::create_dir_all(path)
                .change_context(VirtualEnvError::CreatingError)
                .attach_printable("Unable to create root directory for virtual env");
            info!("Created root dir");
        }

        let venv_path = path.join(name);
        info!(
            "Executing {} -m venv {}",
            python,
            &venv_path.as_path().display()
        );
        let status = process::Command::new(python)
            .arg("-m")
            .arg("venv")
            .arg(venv_path)
            .status()
            .change_context(VirtualEnvError::CreatingError)?;

        if status.code().unwrap_or_default() > 0 {
            return Err(
                Report::new(VirtualEnvError::CreatingError).attach_printable(format!(
                    "Error creating venv {}: ",
                    path.join(name).as_path().display()
                )),
            );
        }

        info!("Created virtual environment {name}");
        Ok(())
    }

    pub fn delete(&self, name: String) -> Result<(), VirtualEnvError> {
        if !self.list().contains(&name) {
            error!("Virtual environment `{name}` is not found");
            return Err(Report::new(VirtualEnvError::NotVirtualEnv(name.clone()))
                .attach_printable(format!(
                    "Cannot delete virtual environment: {} is not managed by rsenv",
                    name
                )));
        }
        fs::remove_dir_all(self.root_dir()?.join(name.clone()).as_path())
            .change_context(VirtualEnvError::IOError)
            .attach_printable("Unable to delete virtual env")?;
        info!("Deleted venv {}", name);
        Ok(())
    }
}

impl VirtualEnvCompatible for Rsenv {
    fn root_dir(&self) -> Result<PathBuf, VirtualEnvError> {
        let expanded = shellexpand::full(
            &SETTINGS
                .read()
                .map_err(|_| Report::new(VirtualEnvError::ConfigurationError))
                .attach_printable("unable to read path from SETTINGS")?
                .path,
        )
        .change_context(VirtualEnvError::ConfigurationError)
        .attach_printable("unable to expand SETTINGS.path to the actual path")?
        .to_string();
        Ok(Path::new(&expanded).to_path_buf().join("versions"))
    }

    fn list(&self) -> HashSet<String> {
        if let Ok(root) = self.root_dir() {
            let mut venvs = get_venvs_by_glob("*/*".into(), &root).unwrap_or_default();
            venvs.extend(get_venvs_by_glob("*".into(), &root).unwrap_or_default());
            return venvs;
        }
        HashSet::new()
    }

    fn relevant(&self) -> bool {
        if std::env::current_dir().is_err() {
            return false;
        }
        if let Ok(metadata) =
            fs::symlink_metadata(std::env::current_dir().unwrap().join(".python-virtualenv"))
        {
            return metadata.is_file();
        }
        false
    }

    fn venv_name(&self) -> Result<std::string::String, VirtualEnvError> {
        Ok(
            fs::read_to_string(get_current_dir()?.join(".python-virtualenv"))
                .change_context(VirtualEnvError::IOError)
                .attach_printable("Unable to read .python-virtualenv")?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<(), VirtualEnvError> {
        let _ = File::create(".python-virtualenv")
            .change_context(VirtualEnvError::IOError)
            .attach_printable("Unable to create .python-virtualenv")?
            .write_all(name.as_bytes());
        info!("Saved changes to .python-virtualenv");
        fs::remove_file(".python-version").unwrap_or_default();
        Ok(())
    }
}
