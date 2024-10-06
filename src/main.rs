mod cli;
mod display;
mod frame;

use clap::{Parser};
use crate::cli::{Commands, Cli};
use crate::frame::Frame;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(add_command) => {
            println!("{} added.", add_command.game);
        }
        Commands::Start(start_command) => {
            println!("Memory Allocated: {} KB", start_command.memory);
            println!("Starting...");

            Frame::new(None, None).run();
        }
    }
}
