mod cli;
mod display;
mod frame;
mod opcode;

use clap::{Parser};
use crate::cli::{Commands, Cli};
use crate::frame::Frame;
use std::fs;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_command) => {
            match fs::metadata(&add_command.game) {
                Ok(metadata) => {
                    if let Err(e) = fs::create_dir_all("./src/games") {
                        eprintln!("Failed to create directory: {}", e);
                        return;
                    }

                    if metadata.is_file(){
                        fs::rename(&add_command.game, format!("./src/games/{}", add_command.game)).expect("Failed to move game file");
                        println!("{} added.", add_command.game);
                    }
                }
                Err(e) => { println!("{}", e); }
            }
        }
        Commands::Start(start_command) => {
            println!("Memory Allocated: {} KB", start_command.memory);
            println!("Starting...");

            Frame::new(None, None).run();
        }
    }
}
