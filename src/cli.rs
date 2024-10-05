use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Chip8-Emulator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartCommand),
    Add(AddCommand),
}

#[derive(Parser)]
pub struct StartCommand{
    #[arg(
        short,
        long,
        default_value = "128",
        value_parser = clap::value_parser!(u32).range(8..=4096),
        help = "Amount of memory for the emulator."
    )]
    pub memory: u32,
}

#[derive(Parser)]
pub struct AddCommand{
    #[arg(short, long)]
    pub game: String,
}
