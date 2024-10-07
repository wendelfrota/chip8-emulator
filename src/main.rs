mod cli;
mod display;
mod frame;
mod opcode;
mod cpu;
mod commands;

use clap::Parser;
use crate::cli::{Commands, Cli};


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(add_command) => commands::handle_add_command(&add_command),
        Commands::Start => commands::handle_start_command(),
    }
}
