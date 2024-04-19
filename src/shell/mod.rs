mod bash;
mod fish;
mod zsh;

use serde::Serialize;
use simplelog::{debug, error};
use std::path::{Path, PathBuf};

use std::os::unix::process::parent_id;
use sysinfo::{Pid, System};
use tinytemplate::TinyTemplate;

use error_stack::{Report, Result, ResultExt};

use crate::errors::VirtualEnvError;

#[derive(Debug, PartialEq)]
pub enum SupportedShell {
    Zsh,
    Bash,
    Fish,
}

#[derive(Serialize)]
struct ActivateTemplateContext {
    activate_path: String,
    current_directory: String,
}

#[derive(Serialize)]
struct DeactivateTemplateContext {
    forced: bool,
}

impl SupportedShell {
    pub fn new() -> Result<Self, VirtualEnvError> {
        let mut system = System::new();
        system.refresh_processes();
        let name = match system.process(Pid::from_u32(parent_id())) {
            Some(parent_process) => parent_process.name(),
            None => {
                error!("Unable to detect parent process");
                return Err(Report::new(VirtualEnvError::ShellDetectionError));
            }
        };
        debug!("Parent process: {name:?}");
        SupportedShell::from_name(name)
    }

    fn from_name(input: &str) -> Result<SupportedShell, VirtualEnvError> {
        match input {
            "zsh" => Ok(SupportedShell::Zsh),
            "bash" => Ok(SupportedShell::Bash),
            "fish" => Ok(SupportedShell::Fish),
            _ => Err(Report::new(VirtualEnvError::ShellDetectionError)),
        }
    }

    pub fn get_hook(&self) -> &str {
        match &self {
            SupportedShell::Zsh => zsh::HOOK,
            SupportedShell::Fish => fish::HOOK,
            _ => bash::HOOK,
        }
    }
    fn get_activate_template(&self) -> &str {
        match self {
            SupportedShell::Fish => fish::ACTIVATE_TEMPLATE,
            _ => bash::ACTIVATE_TEMPLATE,
        }
    }
    pub(crate) fn get_config_path(&self) -> Result<String, VirtualEnvError> {
        let config = match self {
            SupportedShell::Fish => fish::CONFIG,
            SupportedShell::Zsh => zsh::CONFIG,
            SupportedShell::Bash => bash::CONFIG,
        };
        let result = shellexpand::full(config)
            .change_context(VirtualEnvError::ConfigurationError)
            .attach_printable("Incorrect config file")?
            .into_owned();
        Ok(result)
    }

    pub(crate) fn get_init_command(&self) -> &str {
        match self {
            SupportedShell::Fish => fish::INIT_COMMAND,
            _ => bash::INIT_COMMAND,
        }
    }

    fn get_deactivate_template(&self) -> &str {
        match self {
            SupportedShell::Fish => fish::DEACTIVATE_TEMPLATE,
            _ => bash::DEACTIVATE_TEMPLATE,
        }
    }

    fn get_activate_path(&self, venv_root: &Path) -> PathBuf {
        match self {
            SupportedShell::Fish => venv_root.join("bin").join("activate.fish"),
            _ => venv_root.join("bin").join("activate"),
        }
    }

    pub fn render_activate(
        &self,
        venv_root: PathBuf,
        current_path: PathBuf,
    ) -> Result<String, VirtualEnvError> {
        let context = ActivateTemplateContext {
            activate_path: format!("{}", &self.get_activate_path(&venv_root).display()),
            current_directory: format!("{}", &current_path.display()),
        };
        let mut tt = TinyTemplate::new();
        tt.add_template("activate", self.get_activate_template())
            .change_context(VirtualEnvError::TemplateRenderError)?;
        tt.render("activate", &context)
            .change_context(VirtualEnvError::TemplateRenderError)
    }

    pub fn render_deactivate(&self, forced: bool) -> Result<String, VirtualEnvError> {
        let context = DeactivateTemplateContext { forced };
        let mut tt = TinyTemplate::new();
        tt.add_template("deactivate", self.get_deactivate_template())
            .change_context(VirtualEnvError::TemplateRenderError)?;
        tt.render("deactivate", &context)
            .change_context(VirtualEnvError::TemplateRenderError)
    }
}
