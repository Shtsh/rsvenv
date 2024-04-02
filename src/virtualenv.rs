use crate::configuration::SETTINGS;
use simplelog::{debug, error, info};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io, process};

#[derive(Debug)]
pub struct Local;
#[derive(Debug)]
pub struct Rsenv;
#[derive(Debug)]
pub struct Pyenv;

fn is_virtualenv(path: &Path) -> bool {
    let activate_path = path.join("bin").join("activate");
    match fs::metadata(activate_path) {
        Err(_) => false,
        Ok(metadata) => metadata.is_file(),
    }
}

impl Pyenv {
    fn try_list_root_dir(dir: PathBuf) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut result: HashSet<String> = HashSet::new();
        let dir_str = dir.into_os_string().into_string().unwrap_or_default();
        for path in glob::glob(format!("{}/*/envs/*/", &dir_str).as_str())?.flatten() {
            let value = path.strip_prefix(&dir_str)?.to_str().unwrap_or_default();
            if is_virtualenv(&path) {
                result.insert(String::from(value));
            }
        }
        Ok(result)
    }
}

impl Rsenv {
    fn try_list_root_dir(dir: PathBuf) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut result: HashSet<String> = HashSet::new();
        for venv in dir.read_dir()? {
            let entry = venv?;
            if is_virtualenv(&entry.path()) {
                result.insert(entry.file_name().into_string().unwrap());
            }
        }
        Ok(result)
    }

    pub fn create(&self, name: &String, python: &String) -> Result<(), Box<dyn Error>> {
        if self.list().contains(name) {
            return Err(format!("Virtual environment: {} already exists", name).into());
        }

        let root_dir = self.root_dir()?;

        let path = root_dir.as_path();
        if !path.exists() {
            fs::create_dir_all(path)?;
            info!("Created root dir");
        }

        let venv_path = path.join(name);
        let path_str = venv_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_default();
        info!("Executing {} -m venv {}", python, path_str);
        let status = process::Command::new(python)
            .arg("-m")
            .arg("venv")
            .arg(venv_path)
            .status()?;

        if status.code().unwrap_or_default() > 0 {
            error!("Error creating venv {}", status);
            return Err(format!(
                "Error creating venv: python executable status code is {}",
                status
            )
            .into());
        }

        info!("Created virtual environment {}", name);
        Ok(())
    }

    pub fn delete(&self, name: &String) -> Result<(), Box<dyn Error>> {
        if !self.list().contains(name) {
            return Err(format!(
                "Cannot delete virtual environment: {} is not managed by rsenv",
                name
            )
            .into());
        }
        fs::remove_dir_all(self.root_dir()?.join(name).as_path())?;
        info!("Deleted venv {}", name);
        Ok(())
    }
}

pub trait VirtualEnvCompatible {
    fn root_dir(&self) -> Result<PathBuf, Box<dyn Error>>;

    fn list(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn relevant(&self) -> bool;

    fn get_name_from_cwd(&self) -> Result<String, Box<dyn Error>>;

    fn path_from_name(&self, name: &str) -> Result<PathBuf, Box<dyn Error>> {
        let venv_path = self.root_dir()?.join(name);
        if is_virtualenv(&venv_path) {
            return Ok(venv_path);
        }
        Err(format!("Unable to find venv {}", name).into())
    }

    fn path(&self) -> Result<PathBuf, Box<dyn Error>> {
        self.path_from_name(&self.get_name_from_cwd()?)
    }

    fn save(&self, _name: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl VirtualEnvCompatible for Local {
    fn root_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(std::env::current_dir()?)
    }

    fn relevant(&self) -> bool {
        self.get_name_from_cwd().is_ok()
    }

    fn get_name_from_cwd(&self) -> Result<String, Box<dyn Error>> {
        let current_path = std::env::current_dir()?;
        for local_venv_path in ["venv", ".venv", "virtualenv", ".virtualenv"] {
            if is_virtualenv(&current_path.join(local_venv_path)) {
                return Ok(local_venv_path.to_string());
            }
        }
        Err("Unable to detect local venv".into())
    }
}

impl VirtualEnvCompatible for Pyenv {
    fn root_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        let root = std::env::var("PYENV_ROOT").unwrap_or("~/.pyenv".to_string());
        let expanded = shellexpand::full(&root)?.to_string();
        Ok(Path::new(&expanded).to_path_buf().join("versions"))
    }

    fn list(&self) -> HashSet<String> {
        if let Ok(root) = self.root_dir() {
            return Pyenv::try_list_root_dir(root).unwrap_or_default();
        }
        HashSet::new()
    }

    fn relevant(&self) -> bool {
        if let Ok(metadata) =
            fs::symlink_metadata(std::env::current_dir().unwrap().join(".python-version"))
        {
            if metadata.is_file() {
                debug!("Found .python-version");
                return true;
            }
        }
        false
    }

    fn get_name_from_cwd(&self) -> Result<String, Box<dyn Error>> {
        Ok(
            fs::read_to_string(std::env::current_dir()?.join(".python-version"))?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(".python-version")?;
        file.write_all(name.as_bytes())?;
        info!("Saved changes to .python-version");
        fs::remove_file(".python-virtualenv").unwrap_or_default();
        Ok(())
    }
}

impl VirtualEnvCompatible for Rsenv {
    fn root_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        let expanded = shellexpand::full(&SETTINGS.read()?.path)?.to_string();
        Ok(Path::new(&expanded).to_path_buf().join("versions"))
    }

    fn list(&self) -> HashSet<String> {
        if let Ok(root) = self.root_dir() {
            return Rsenv::try_list_root_dir(root).unwrap_or_default();
        }
        HashSet::new()
    }

    fn relevant(&self) -> bool {
        if let Ok(metadata) =
            fs::symlink_metadata(std::env::current_dir().unwrap().join(".python-virtualenv"))
        {
            return metadata.is_file();
        }
        false
    }

    fn get_name_from_cwd(&self) -> Result<String, Box<dyn Error>> {
        Ok(
            fs::read_to_string(std::env::current_dir()?.join(".python-virtualenv"))?
                .trim()
                .to_string(),
        )
    }

    fn save(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(".python-virtualenv")?;
        file.write_all(name.as_bytes())?;
        info!("Saved changes to .python-virtualenv");
        fs::remove_file(".python-version").unwrap_or_default();
        Ok(())
    }
}

pub struct VirtualEnvironment<'a> {
    // Venv path
    pub kind: &'a dyn VirtualEnvCompatible,
}

impl VirtualEnvironment<'_> {
    pub fn detect() -> Result<Self, Box<dyn Error>> {
        if Rsenv.relevant() {
            return Ok(Self { kind: &Rsenv });
        }

        if Pyenv.relevant() {
            return Ok(Self { kind: &Pyenv });
        }

        if Local.relevant() {
            return Ok(Self { kind: &Local });
        }

        Err("Unable to detect venv".into())
    }

    pub fn activate(&self, venv_name: Option<&String>) -> Result<(), Box<dyn Error>> {
        let path = match venv_name {
            Some(value) => self.kind.path_from_name(value)?,
            None => self.kind.path()?,
        };
        info!("Activating {path:?}");
        let activate_path = path.join("bin").join("activate");
        match fs::symlink_metadata(&activate_path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    // activate
                    let mut command: String = "\nsource ".to_owned();
                    command.push_str(
                        activate_path
                            .to_str()
                            .ok_or("Unable to convert activate_path to str")?,
                    );
                    command.push("\n".parse()?);
                    // set variables
                    command.push_str("export RSVENV_ACTIVATE_PATH=");
                    command.push_str(
                        std::env::current_dir()?
                            .to_str()
                            .ok_or("Unable to convert current_path to str")?,
                    );
                    command.push("\n".parse()?);

                    io::stdout().write_all(command.as_bytes())?;
                }
            }
            Err(_) => {
                error!("Cannot activate script {path:?}")
            }
        }
        Ok(())
    }

    pub fn deactivate(manual: bool) -> Result<(), Box<dyn Error>> {
        match std::env::var("VIRTUAL_ENV") {
            Ok(value) => {
                info!("Deactivating {value:?}");
                io::stdout().write_all("unset RSVENV_DEACTIVATE_PATH\n".as_bytes())?;
                io::stdout().write_all("deactivate\n".as_bytes())?;
                match std::env::var("RSVENV_ACTIVATE_PATH") {
                    Ok(_) => {
                        if manual {
                            io::stdout().write_all(
                                "export RSVENV_DEACTIVATE_PATH=$RSVENV_ACTIVATE_PATH\n".as_bytes(),
                            )?;
                        }
                    }
                    Err(_) => {
                        info!("Virtual env was activated outside of rsvenv")
                    }
                }
            }
            Err(_) => {
                info!("Virtual env is not active")
            }
        }
        Ok(())
    }
}
