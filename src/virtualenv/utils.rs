use std::{fs, path::PathBuf};

use error_stack::{Report, Result, ResultExt};

use crate::errors::VirtualEnvError;

pub fn is_virtualenv(path: &PathBuf) -> Result<(), VirtualEnvError> {
    if fs::metadata(path.join("bin").join("activate")).is_ok_and(|x| x.is_file()) {
        Ok(())
    } else {
        Err(Report::new(VirtualEnvError::NotVirtualEnv(
            path.to_string_lossy().to_string(),
        )))
    }
}

pub fn get_current_dir() -> Result<PathBuf, VirtualEnvError> {
    std::env::current_dir()
        .attach_printable("Unable to get current dir")
        .change_context(VirtualEnvError::VenvBuildError)
}
