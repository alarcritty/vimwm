mod cli;
mod commands;
mod config;
mod daemon;
mod generators;
mod helpers;

use cli::{Cli, Command};
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Start => commands::start(),
        Command::Stop => commands::stop(),
        Command::Restart => commands::restart(),
        Command::Status => commands::status(),
        Command::Reload => commands::reload(),
        Command::Bind { key, action } => commands::bind(&key, &action),
        Command::Unbind { key } => commands::unbind(&key),
        Command::Layout { mode } => commands::layout(&mode),
        Command::Gaps { size } => commands::gaps(size),
        Command::Padding { size } => commands::padding(size),
        Command::Spaces => commands::spaces(),
        Command::Preset { name } => commands::preset(&name),
        Command::Config { action } => commands::config_cmd(&action),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
