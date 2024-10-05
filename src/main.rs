mod cli;
use clap::{Parser};
use crate::cli::{Commands, Cli};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(add_command) => {
            println!("{} added.", add_command.game);
        }
        Commands::Start(start_command) => {
            println!("Memory Allocated: {} KB", start_command.memory);
            println!("Starting...")
        }
    }
}
