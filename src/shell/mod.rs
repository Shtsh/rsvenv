mod bash;
mod zsh;

use simplelog::debug;
use std::str::FromStr;
use std::{fmt::Display, os::unix::process::parent_id};
use sysinfo::{Pid, System};

use error_stack::{Context, Report, Result};

#[derive(Debug, PartialEq)]
pub enum SupportedShells {
    Zsh,
    Bash,
    Fish,
}

#[derive(Debug)]
pub struct ShellDetectionError {}

impl Context for ShellDetectionError {}

impl Display for ShellDetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unable to detect shell")
    }
}

impl FromStr for SupportedShells {
    type Err = ShellDetectionError;

    fn from_str(input: &str) -> std::result::Result<SupportedShells, Self::Err> {
        match input {
            "zsh" => Ok(SupportedShells::Zsh),
            "bash" => Ok(SupportedShells::Bash),
            "fish" => Ok(SupportedShells::Fish),
            _ => Err(ShellDetectionError {}),
        }
    }
}

pub trait Hook {
    fn get_hook(&self) -> &str;
}

impl Hook for SupportedShells {
    fn get_hook(&self) -> &str {
        match &self {
            SupportedShells::Zsh => zsh::HOOK,
            _ => bash::HOOK,
        }
    }
}

pub fn detect_shell() -> Result<SupportedShells, ShellDetectionError> {
    let mut system = System::new();
    system.refresh_processes();
    let name = match system.process(Pid::from_u32(parent_id())) {
        Some(parent_process) => parent_process.name(),
        None => return Err(Report::new(ShellDetectionError {})),
    };
    debug!("Parent process: {name:?}");
    Ok(SupportedShells::from_str(name)?)
}
