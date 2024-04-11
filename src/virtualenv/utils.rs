use std::{collections::HashSet, fs, path::PathBuf};

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

pub fn get_venvs_by_glob(glob: String, dir: &PathBuf) -> Result<HashSet<String>, VirtualEnvError> {
    let mut result = HashSet::new();
    for path in glob::glob(format!("{}/{glob}", &dir.as_path().display()).as_str())
        .attach_printable(format!("Unable to parse glob {glob}"))
        .change_context(VirtualEnvError::VenvBuildError)?
        .flatten()
    {
        let value = path
            .strip_prefix(&dir)
            .attach_printable("Unable to strip prefix")
            .change_context(VirtualEnvError::VenvBuildError)?
            .to_str();
        if is_virtualenv(&path).is_ok() && !value.is_none() {
            result.insert(String::from(value.unwrap()));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir_all;

    use super::*;
    use tempfile;

    #[test]
    fn test_is_virtualenv_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        let bin_dir = dir.as_ref().join("bin");
        fs::create_dir(&bin_dir).unwrap();
        fs::File::create(bin_dir.join("activate")).unwrap();
        let result = is_virtualenv(&dir.as_ref().to_path_buf());
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_virtualenv_error() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = is_virtualenv(&dir.as_ref().to_path_buf());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_venvs_by_glob() {
        let dir = tempfile::TempDir::new().unwrap();
        fs::create_dir(dir.as_ref().join("version")).unwrap();
        create_dir_all(dir.as_ref().join("version").join("v1").join("bin")).unwrap();
        create_dir_all(dir.as_ref().join("version").join("v2").join("bin")).unwrap();
        create_dir_all(dir.as_ref().join("version").join("v3").join("bin")).unwrap();
        fs::File::create(
            dir.as_ref()
                .join("version")
                .join("v1")
                .join("bin")
                .join("activate"),
        )
        .unwrap();
        fs::File::create(
            dir.as_ref()
                .join("version")
                .join("v3")
                .join("bin")
                .join("activate"),
        )
        .unwrap();
        let result = get_venvs_by_glob("*/*".into(), &dir.into_path()).unwrap();
        // v2 is not a valid vitual environment
        let expected = HashSet::from([String::from("version/v1"), String::from("version/v3")]);
        assert_eq!(result, expected)
    }
}
