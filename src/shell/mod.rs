mod bash;
mod zsh;

use simplelog::debug;
use std::os::unix::process::parent_id;
use std::str::FromStr;
use sysinfo::{Pid, System};

#[derive(Debug, PartialEq)]
pub enum SupportedShells {
    Zsh,
    Bash,
    Fish,
}

impl FromStr for SupportedShells {
    type Err = ();
    fn from_str(input: &str) -> Result<SupportedShells, Self::Err> {
        match input {
            "zsh" => Ok(SupportedShells::Zsh),
            "bash" => Ok(SupportedShells::Bash),
            "fish" => Ok(SupportedShells::Fish),
            _ => Err(()),
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

pub fn detect_shell() -> SupportedShells {
    let mut system = System::new();
    system.refresh_processes();
    let parent_process = system.process(Pid::from_u32(parent_id()));
    let name = parent_process.unwrap().name();
    debug!("Parent process: {name:?}");
    SupportedShells::from_str(name).unwrap()
}
