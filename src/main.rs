mod cli;
mod commands;
mod cpu;
mod emulator;
mod opcode;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(add_command) => commands::handle_add_command(&add_command),
        Commands::Start => commands::handle_start_command(),
    }
}
