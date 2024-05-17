use std::{path::PathBuf, process, str::FromStr};

use anyhow::{bail, Context, Result};
use glob::Pattern;
use simplelog::info;

pub struct PythonInterpreter<'a> {
    pub version: String,
    pub interpreter: &'a String,
}

impl<'a> PythonInterpreter<'a> {
    pub fn new(interpreter: &'a String) -> Result<Self> {
        let version = PythonInterpreter::detect_version(interpreter)?;
        Ok(PythonInterpreter {
            version,
            interpreter,
        })
    }

    pub fn create_venv(&self, path: &PathBuf) -> Result<()> {
        info!(
            "Executing {} -m venv {}",
            self.interpreter,
            &path.as_path().display()
        );
        let status = process::Command::new(self.interpreter)
            .arg("-m")
            .arg("venv")
            .arg(path)
            .status()
            .context("Error spawning interpreter")?;

        if status.code().unwrap_or_default() > 0 {
            bail!("Error creating venv {}: ", path.as_path().display());
        }

        Ok(())
    }

    fn detect_version(interpreter: &String) -> Result<String> {
        info!("Detecting python version");
        let output = process::Command::new(interpreter)
            .arg("-c")
            .arg(r#"import platform; print(platform.python_version())"#)
            .output()
            .context("Unable to spawn interpreter")?;
        if output.status.code().unwrap_or(1) != 0 {
            bail!("Python executable returned {}", output.status);
        }
        let mut version =
            String::from_utf8(output.stdout).context("unable to read version from stdout")?;

        version = version.trim().into();
        if !Pattern::from_str("*.*.*").unwrap().matches(&version) {
            bail!("Unexpected python version {}", &version);
        }

        Ok(version)
    }
}
