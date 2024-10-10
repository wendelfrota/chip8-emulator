use std::{fs, io};
use std::path::Path;
use crate::constants::*;
use crate::cpu::CPU;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct Emulator {
    window: Option<Window>,
    pixels: Option<Pixels>,
    input: WinitInputHelper,
    cpu: CPU,
}

impl Emulator {
    pub fn new() -> Self {
        let input = WinitInputHelper::new();

        Emulator {
            window: None,
            pixels: None,
            input,
            cpu: CPU::new(),
        }
    }

    pub fn start(&mut self, mut event_loop: EventLoop<()>) -> Result<(), String> {
        let game: String;

        match Self::select_game() {
            Ok(path) => game = path,
            Err(e) => return Err(e.to_string()),
        }
        
        self.cpu.load_to_memory(&game).expect("Failed to load game");

        if self.window.is_none() {
            self.window = Some(WindowBuilder::new()
                .with_title("Chip8 Emulator")
                .with_inner_size(winit::dpi::PhysicalSize::new(
                    CHIP8_WIDTH * SCALE_FACTOR,
                    CHIP8_HEIGHT * SCALE_FACTOR,
                ))
                .build(&event_loop)
                .expect("Failed to create window"));
        }

        if self.pixels.is_none() {
            let window = self.window.as_ref().unwrap();
            self.pixels = Some(Pixels::new(
                CHIP8_WIDTH,
                CHIP8_HEIGHT,
                SurfaceTexture::new(
                    CHIP8_WIDTH * SCALE_FACTOR,
                    CHIP8_HEIGHT * SCALE_FACTOR,
                    window,
                ),
            ).expect("Failed to create pixels."));
        }

        let mut input = self.input.clone();
        let mut cpu = self.cpu.clone();
        let window = self.window.take().unwrap();
        let mut pixels = self.pixels.take().unwrap();

        event_loop.run_return(move |event, _, control_flow| {
            if input.update(&event) {
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            match event {
                Event::RedrawRequested(_) => {
                    self.run_cycle(&mut cpu, &mut pixels);

                    if let Err(e) = pixels.render() {
                        eprintln!("pixels.render() failed: {}", e);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
        Ok(())
    }

    fn run_cycle(&mut self, cpu: &mut CPU, pixels: &mut Pixels) {
        self.cpu.execute_cycle();
        Self::draw(cpu, pixels);
    }

    fn draw(cpu: &CPU, pixels: &mut Pixels) {
        let frame = pixels.frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % CHIP8_WIDTH as usize;
            let y = i / CHIP8_WIDTH as usize;
            let color = if cpu.display[y * CHIP8_WIDTH as usize + x] {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };
            pixel.copy_from_slice(&color);
        }
    }

    fn select_game() -> Result<String, io::Error> {
        let games_dir = Path::new("./src/games");
        let entries = fs::read_dir(games_dir)?;
        let mut games = Vec::new();

        println!("Available games:");
        for (index, entry) in entries.enumerate() {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            println!("{}. {}", index + 1, file_name);
            games.push(entry.path());
        }

        let games_len = games.len();

        if games_len == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No games found"));
        }

        println!("Enter the number of the game you want to play:");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(choice) if choice > 0 && choice <= games_len => {
                    break Ok(games[choice - 1].to_string_lossy().into_owned())
                },
                _ => println!("Please select a valid game [1-{games_len}]"),
            }
        }
    }

    pub fn clear(&mut self) {
        println!("Clearing screen");
    }
}
