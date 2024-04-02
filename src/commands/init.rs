use crate::shell::{detect_shell, Hook};
use clap::Parser;
use std::io;
use std::io::Write;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn execute(&self) {
        let shell = detect_shell();
        let hook = shell.get_hook();

        io::stdout().write_all(hook.as_bytes()).unwrap();
    }
}
