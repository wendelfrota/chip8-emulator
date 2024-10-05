<h1 align="center">Chip-8 Emulator</h1>

## Overview
This project is a **Chip-8 emulator** developed for learning about low-level programming and the Rust programming 
language. Although it's a small project, it aims to provide a significant understanding of how emulators work and how 
to interact with hardware at a low level.


## Features
- **Command Line Interface (CLI)**: Interact with the emulator via terminal commands.
- **Memory Management**: Allocate memory for the emulator with a configurable size, allowing you to learn about memory 
handling.
- **Game Support**: Start the emulator with preloaded Chip-8 games and the ability to add custom games.


## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/wendelfrota/chip8-emulator
   cd chip8_emulator
   ```

2. Build the project:
   ```bash
   cargo build
   ```


## Usage
### Start the Emulator
To start the emulator with a specified memory size:

```bash
cargo run -- start --memory <size>
```
Replace `<size>` with the desired memory size (e.g., `128`). If no size is provided, the emulator will use a default 
memory allocation.


### Add a Game
To add a compatible Chip-8 game to the emulator:

```bash
cargo run -- add --game <game>
```
Replace `<game>` with the path of the game you want to add.


## Commands
- `start`: Initializes the emulator with a specified memory size and starts preloaded games.
- `add`: Adds a custom Chip-8 game to the emulator.


## Development
Feel free to contribute to the project by opening issues or submitting pull requests. 
This project is designed for educational purposes, and contributions are welcome!


## License
This project is licensed under the MIT License.
