use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

pub fn is_virtualenv(path: &Path) -> Result<()> {
    if fs::metadata(path.join("bin").join("activate")).is_ok_and(|x| x.is_file()) {
        Ok(())
    } else {
        bail!("{} is not a virtual environment", path.display());
    }
}

pub fn get_current_dir() -> Result<PathBuf> {
    std::env::current_dir().context("Unable to get current dir")
}

pub fn get_venvs_by_glob(glob: String, dir: &PathBuf) -> Result<HashSet<String>> {
    let mut result = HashSet::new();
    for path in glob::glob(format!("{}/{glob}", &dir.as_path().display()).as_str())
        .with_context(|| format!("Unable to parse glob {glob}"))?
        .flatten()
    {
        let value = path
            .strip_prefix(dir)
            .context("Unable to strip prefix")?
            .to_str();
        if let Some(unwrapped) = value {
            if is_virtualenv(&path).is_ok() {
                result.insert(String::from(unwrapped));
            }
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
