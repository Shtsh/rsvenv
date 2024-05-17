use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

use crate::shell::SupportedShell;
use anyhow::{bail, Context, Result};
use clap::Parser;
use simplelog::{debug, info};

#[derive(Debug, Parser)]
pub struct Command {}

fn find_occurence(filename: &str, substr: &str) -> Result<bool> {
    for line in read_to_string(filename)
        .context("Unable to read config file")?
        .lines()
    {
        if line.contains(substr) {
            return Ok(true);
        }
    }
    Ok(false)
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        let shell = SupportedShell::new()?;

        let config_path = shell.get_config_path()?;

        debug!("Installing to {config_path}");

        let init_line = shell.get_init_command();

        if find_occurence(&config_path, init_line)? {
            bail!("RSVENV already installed to config");
        };
        let mut file = OpenOptions::new()
            .append(true)
            .open(config_path)
            .context("Unable to open config file for appending")?;

        file.write_all(init_line.as_bytes())
            .context("Error when writing to the file")?;

        info!("Successfully modified shell config. Please restart you session");
        Ok(())
    }
}
