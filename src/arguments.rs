use clap::{Parser, Subcommand};
use error_stack::Result;
use simplelog::debug;

use crate::errors::CommandExecutionError;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(name = "install", about = "Configure the shell to use RSVENV")]
    Install(crate::commands::install::Command),
    #[clap(name = "init", about = "Configure the shell environment for pyenv")]
    Init(crate::commands::init::Command),
    #[clap(
        name = "activate",
        about = "Activate a Python virtualenv environment in current shell. Outputs commands to eval in the current shell"
    )]
    Activate(crate::commands::activate::Command),
    #[clap(
        name = "deactivate",
        about = "Deactivate a Python virtual environment. Outputs commands to eval in the current shell"
    )]
    Deactivate(crate::commands::deactivate::Command),
    #[clap(name = "hook", about = "Command to be executed on directory change")]
    Hook(crate::commands::chdir_hook::Command),
    #[clap(name = "list", about = "List existing virtual environments")]
    List(crate::commands::list::ListCommand),
    #[clap(name = "create", about = "Create a new virtual environment")]
    Create(crate::commands::create::CreateCommand),
    #[clap(name = "delete", about = "Delete a virtual environment")]
    Delete(crate::commands::delete::DeleteCommand),
    #[clap(
        name = "use",
        about = "Use the virtual environment in the current directory"
    )]
    Use(crate::commands::use_command::UseCommand),
}

impl Commands {
    pub fn execute(&self) -> Result<(), CommandExecutionError> {
        debug!("executing command");
        match self {
            Commands::Init(command) => command.execute(),
            Commands::Activate(command) => command.execute(),
            Commands::Deactivate(command) => command.execute(),
            Commands::Hook(command) => command.execute(),
            Commands::List(command) => command.execute(),
            Commands::Use(command) => command.execute(),
            Commands::Create(command) => command.execute(),
            Commands::Delete(command) => command.execute(),
            Commands::Install(command) => command.execute(),
        }
    }
}

pub fn parse_commands() -> Cli {
    Cli::parse()
}
