pub mod local;
pub mod pyenv;
pub mod rsenv;
pub mod traits;
mod utils;

use error_stack::{Result, ResultExt};
use simplelog::info;

use std::io;
use std::io::Write;

use crate::errors::VirtualEnvError;
use crate::virtualenv::utils::{get_current_dir, is_virtualenv};

use self::local::Local;
use self::pyenv::Pyenv;
use self::rsenv::Rsenv;
use self::traits::VirtualEnvCompatible;

pub struct VirtualEnvironment<'a> {
    // Venv path
    pub kind: &'a dyn VirtualEnvCompatible,
}

impl VirtualEnvironment<'_> {
    pub fn detect() -> Option<Self> {
        if Rsenv.relevant() {
            return Some(Self { kind: &Rsenv });
        }

        if Pyenv.relevant() {
            return Some(Self { kind: &Pyenv });
        }

        if Local.relevant() {
            return Some(Self { kind: &Local });
        }

        None
    }

    pub fn activate(&self, venv_name: Option<&String>) -> Result<(), VirtualEnvError> {
        let path = self.kind.path(venv_name)?;
        is_virtualenv(&path)?;

        info!("Activating {path:?}");
        let command = format!(
            r#"
source {}
export RSVENV_ACTIVATE_PATH={}
"#,
            path.join("bin").join("activate").as_path().display(),
            get_current_dir()?.as_path().display()
        );

        io::stdout()
            .write_all(command.as_bytes())
            .attach_printable("Unable to write to STDOUT")
            .change_context(VirtualEnvError::IOError)?;

        Ok(())
    }

    pub fn deactivate(manual: bool) -> Result<(), VirtualEnvError> {
        let value =
            std::env::var("VIRTUAL_ENV").change_context(VirtualEnvError::VenvIsNotActive)?;
        info!("Deactivating {value:?}");

        let mut command = "\nunset RSVENV_DEACTIVATE_PATH\ndeactivate\n".to_owned();

        if manual {
            command.push_str("export RSVENV_DEACTIVATE_PATH=$RSVENV_ACTIVATE_PATH\n");
        }

        io::stdout()
            .write_all(command.as_bytes())
            .attach_printable("Unable to write to STDOUT")
            .change_context(VirtualEnvError::IOError)?;

        Ok(())
    }
}
