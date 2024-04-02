use crate::virtualenv::VirtualEnvironment;
use clap::Parser;
use simplelog::debug;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    fn should_activate(&self, just_deactivated: bool) -> bool {
        if let Ok(value) = std::env::var("RSVENV_DEACTIVATE_PATH") {
            let disabled_in_path = Path::new(&value);
            let current_path = std::env::current_dir().unwrap();
            if current_path.starts_with(disabled_in_path) {
                debug!("Disabled manually in the parent directory");
                return false;
            };
        }
        if std::env::var("VIRTUAL_ENV").is_ok() {
            return just_deactivated;
        }

        true
    }

    fn should_deactivate(&self) -> bool {
        if std::env::var("VIRTUAL_ENV").is_err() {
            return false;
        }
        match std::env::var("RSVENV_ACTIVATE_PATH") {
            Ok(value) => {
                let activated_in_path = Path::new(&value);
                let current_path = std::env::current_dir().unwrap();
                if current_path.starts_with(activated_in_path) {
                    return false;
                };
                debug!("Disabling venv {value:?}");
                true
            }
            Err(_) => false,
        }
    }

    pub fn execute(&self) {
        let deactivated = self.should_deactivate();
        if deactivated {
            VirtualEnvironment::deactivate(false).unwrap_or(());
        }
        if self.should_activate(deactivated) {
            if let Ok(method) = VirtualEnvironment::detect() {
                method.activate(None).unwrap_or(())
            }
        }
    }
}
