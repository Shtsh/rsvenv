use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

use crate::errors::CommandExecutionError;
use crate::shell::SupportedShell;
use clap::Parser;
use error_stack::{Report, Result, ResultExt};
use simplelog::{debug, error, info};

#[derive(Debug, Parser)]
pub struct Command {}

fn find_occurence(filename: &str, substr: &str) -> Result<bool, CommandExecutionError> {
    for line in read_to_string(filename)
        .change_context(CommandExecutionError {
            command: "install".into(),
        })
        .attach_printable("Unable to read config file")?
        .lines()
    {
        if line.contains(substr) {
            return Ok(true);
        }
    }
    Ok(false)
}

impl Command {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        let error_context = CommandExecutionError {
            command: "install".into(),
        };
        let shell = SupportedShell::new()
            .change_context(error_context.clone())
            .attach("Unable to detect current shell")?;

        let config_path = shell
            .get_config_path()
            .change_context(error_context.clone())?;

        debug!("Installing to {config_path}");

        let init_line = shell.get_init_command();

        if find_occurence(&config_path, init_line)? {
            error!("RSVENV already installed to config");
            return Err(Report::new(CommandExecutionError {
                command: "install".into(),
            }));
        };
        let mut file = OpenOptions::new()
            .append(true)
            .open(config_path)
            .change_context(error_context.clone())
            .attach("Unable to open config file for appending")?;

        file.write_all(init_line.as_bytes())
            .change_context(error_context)
            .attach_printable("Error when writing to the file")?;

        info!("Successfully modified shell config. Please restart you session");
        Ok(())
    }
}
