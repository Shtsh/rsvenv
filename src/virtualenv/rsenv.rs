use std::{
    collections::HashSet,
    fs::{self, create_dir_all, File},
    path::{Path, PathBuf},
};

use crate::configuration::SETTINGS;
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use simplelog::info;
use std::io::Write;

use super::{
    python::PythonInterpreter,
    traits::VirtualEnvCompatible,
    utils::{get_current_dir, get_venvs_by_glob},
};

#[derive(Debug)]
pub struct Rsenv;

impl Rsenv {
    pub fn validate_name(name: &str) -> Result<()> {
        if Regex::new(r"^[\w._]*$").unwrap().is_match(name)
            || Regex::new(r"^[\w._]*\/[\w._]*$").unwrap().is_match(name)
        {
            return Ok(());
        }
        bail!("name {name} is invalid");
    }

    pub fn create(&self, name: &String, python: &String) -> Result<()> {
        let interpreter = PythonInterpreter::new(python)?;

        let name_with_version = format!("{}/{}", &interpreter.version, name);
        let existing = self.list();

        Rsenv::validate_name(name)?;
        if existing.contains(name) || existing.contains(&name_with_version) {
            bail!("Virtual environment {name} exists");
        }

        let root_dir = self.root_dir()?;

        let path = root_dir.as_path();
        if !path.exists() {
            create_dir_all(path).context("Unable to create root directory for virtual env")?;
            info!("Created root dir");
        }

        let venv_path = path.join(&interpreter.version).join(name);
        interpreter.create_venv(&venv_path)?;
        info!("Created venv {name_with_version}");
        Ok(())
    }

    pub fn delete(&self, name: String) -> Result<()> {
        Rsenv::validate_name(&name)?;
        if !self.list().contains(&name) {
            bail!(
                "Cannot delete virtual environment: {} is not managed by rsenv",
                name
            );
        }
        fs::remove_dir_all(self.root_dir()?.join(name.clone()).as_path())
            .context("Unable to delete virtual env")?;
        info!("Deleted venv {}", name);
        Ok(())
    }
}

impl VirtualEnvCompatible for Rsenv {
    fn root_dir(&self) -> Result<PathBuf> {
        let expanded = shellexpand::full(
            &SETTINGS
                .read()
                .map_err(|e| anyhow!("Unable to read path from SETTINGS: {e:?}"))?
                .path,
        )
        .context("unable to expand SETTINGS.path to the actual path")?
        .to_string();
        Ok(Path::new(&expanded).to_path_buf().join("venvs"))
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

    fn venv_name(&self) -> Result<String> {
        Ok(
            fs::read_to_string(get_current_dir()?.join(".python-virtualenv"))
                .context("Unable to read .python-virtualenv")?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<()> {
        let _ = File::create(".python-virtualenv")
            .context("Unable to create .python-virtualenv")?
            .write_all(name.as_bytes());
        info!("Saved changes to .python-virtualenv");
        fs::remove_file(".python-version").unwrap_or_default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name_ok() {
        assert!(Rsenv::validate_name(&String::from("good_name")).is_ok());
        assert!(Rsenv::validate_name(&String::from("Good_nAme")).is_ok());
        assert!(Rsenv::validate_name(&String::from("Good_nAme/asdfadsf")).is_ok());
    }
    #[test]
    fn test_bad_name() {
        assert!(Rsenv::validate_name(&String::from("bad!name")).is_err());
        assert!(Rsenv::validate_name(&String::from("Good_nAme/asdfadsf/smth")).is_err());
        assert!(Rsenv::validate_name(&String::from("Good_nAme aa")).is_err());
    }
}
