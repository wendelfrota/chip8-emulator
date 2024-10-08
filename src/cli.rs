use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Chip8-Emulator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Add(AddCommand),
}

#[derive(Parser)]
pub struct AddCommand {
    #[arg(short, long)]
    pub game: String,
}
