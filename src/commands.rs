use crate::cli::AddCommand;
use crate::emulator::Emulator;
use std::fs;

pub fn handle_add_command(add_command: &AddCommand) {
    if let Err(e) = ensure_games_directory() {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    match fs::metadata(&add_command.game) {
        Ok(metadata) if metadata.is_file() => move_game_file(&add_command.game),
        Ok(_) => println!("The specified path is not a file."),
        Err(e) => println!("{}", e),
    }
}

pub fn handle_start_command() {
    println!("Starting...");

    match Emulator::new().start() {
        Ok(_) => println!("Shutting down..."),
        Err(e) => eprintln!("Exiting with error: {}", e),
    }
}

fn move_game_file(game: &str) {
    let destination = format!("./src/games/{}", game);
    if let Err(e) = fs::rename(game, &destination) {
        eprintln!("Failed to move game file: {}", e);
    } else {
        println!("{} added.", game);
    }
}

fn ensure_games_directory() -> std::io::Result<()> {
    fs::create_dir_all("./src/games")
}
