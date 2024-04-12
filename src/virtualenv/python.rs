use std::{path::PathBuf, process, str::FromStr};

use error_stack::{Report, Result, ResultExt};
use glob::Pattern;
use simplelog::info;

use crate::errors::PythonInterpreterError;

pub struct PythonInterpreter<'a> {
    pub version: String,
    pub interpreter: &'a String,
}

impl<'a> PythonInterpreter<'a> {
    pub fn new(interpreter: &'a String) -> Result<Self, PythonInterpreterError> {
        let version = PythonInterpreter::detect_version(interpreter)?;
        Ok(PythonInterpreter {
            version,
            interpreter,
        })
    }

    pub fn create_venv(&self, path: &PathBuf) -> Result<(), PythonInterpreterError> {
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
            .change_context(PythonInterpreterError::CreateVenvError)?;

        if status.code().unwrap_or_default() > 0 {
            return Err(
                Report::new(PythonInterpreterError::CreateVenvError).attach_printable(format!(
                    "Error creating venv {}: ",
                    path.as_path().display()
                )),
            );
        }

        Ok(())
    }

    fn detect_version(interpreter: &String) -> Result<String, PythonInterpreterError> {
        info!("Detecting python version");
        let output = process::Command::new(interpreter)
            .arg("-c")
            .arg(r#"import platform; print(platform.python_version())"#)
            .output()
            .change_context(PythonInterpreterError::UnableToDetectVersion)
            .attach_printable_lazy(|| "{:?}")?;
        if output.status.code().unwrap_or(1) != 0 {
            return Err(Report::new(PythonInterpreterError::UnableToDetectVersion)
                .attach_printable(format!("Python executable returned {}", output.status,)));
        }
        let mut version = String::from_utf8(output.stdout)
            .change_context(PythonInterpreterError::UnableToDetectVersion)
            .attach_printable_lazy(|| "unable to read version from stdout: {:?}")?;

        version = version.trim().into();
        if !Pattern::from_str("*.*.*").unwrap().matches(&version) {
            return Err(Report::new(PythonInterpreterError::UnableToDetectVersion)
                .attach_printable(format!("Unexpected python version {}", &version)));
        }

        Ok(version)
    }
}
