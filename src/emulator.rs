use crate::constants::*;
use crate::cpu::CPU;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct Emulator {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
    input: WinitInputHelper,
    cpu: CPU,
}

impl Emulator {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = WindowBuilder::new()
            .with_title("Chip8 Emulator")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                CHIP8_WIDTH * SCALE_FACTOR,
                CHIP8_HEIGHT * SCALE_FACTOR,
            ))
            .build(&event_loop)
            .unwrap();

        let pixels = Pixels::new(
            CHIP8_WIDTH,
            CHIP8_HEIGHT,
            SurfaceTexture::new(
                CHIP8_WIDTH * SCALE_FACTOR,
                CHIP8_HEIGHT * SCALE_FACTOR,
                &window,
            ),
        )
        .unwrap();

        Emulator {
            event_loop,
            window,
            pixels,
            input,
            cpu: CPU::new(),
        }
    }

    pub fn start(mut self, mut event_loop: EventLoop<()>) -> Result<(), String> {
        event_loop.run_return(move |event, _, control_flow| {
            if self.input.update(&event) {
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            match event {
                Event::RedrawRequested(_) => {
                    self.run_cycle();
                    self.draw();
                    if let Err(e) = self.pixels.render() {
                        eprintln!("pixels.render() failed: {}", e);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => (),
            }
        });
        Ok(())
    }

    fn run_cycle(&mut self) {
        println!("Executing cycle");
    }

    fn draw(&mut self) {
        let frame = self.pixels.get_frame();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % CHIP8_WIDTH as usize;
            let y = i / CHIP8_WIDTH as usize;
            let color = if self.cpu.display[y * CHIP8_WIDTH as usize + x] {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };
            pixel.copy_from_slice(&color);
        }
    }

    pub fn clear(&mut self) {
        println!("Clearing screen");
    }
}
