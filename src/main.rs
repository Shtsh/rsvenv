extern crate simplelog;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
mod arguments;
mod commands;
mod configuration;
mod shell;
mod virtualenv;

fn main() {
    let cli = arguments::parse_commands();
    // configure logging
    let log_level = match configuration::SETTINGS.read().unwrap().verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )])
    .unwrap();
    // execute sub-command
    cli.command.unwrap().execute();
}
