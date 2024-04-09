use core::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum VirtualEnvError {
    VenvBuildError,
    NotVirtualEnv(String),
    VenvIsNotActive,
    AlreadyExists(String),
    CreatingError,
    IOError,
    ConfigurationError,
}

impl fmt::Display for VirtualEnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            VirtualEnvError::NotVirtualEnv(name) => {
                format!("{name} is not a valid virtual environment")
            }
            VirtualEnvError::VenvBuildError => "Unable to detect virtual environment".to_owned(),
            VirtualEnvError::VenvIsNotActive => "Virtual environment is not active".to_owned(),
            VirtualEnvError::IOError => "Unknown I/O error.".to_owned(),
            VirtualEnvError::AlreadyExists(name) => {
                format!("Virtual environment {name} already exists.")
            }
            VirtualEnvError::ConfigurationError => "Configuration error".to_owned(),
            VirtualEnvError::CreatingError => "Error while creating virtual environment".to_owned(),
        };
        f.write_str(&data)
    }
}

impl Context for VirtualEnvError {}

#[derive(Debug, Clone)]
pub struct CommandExecutionError {
    pub command: String,
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Error running command {}", self.command))
    }
}

impl Context for CommandExecutionError {}
