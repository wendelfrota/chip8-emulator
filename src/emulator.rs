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

    pub fn start(mut self, event_loop: EventLoop<()>) -> Result<(), String> {
        event_loop.run(move |event, _, control_flow| {
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
    }

    fn run_cycle(&mut self) {
        println!("Executing cycle");
    }

    fn draw(&mut self) {
        println!("Drawing chip8");
    }

    pub fn clear(&mut self) {
        println!("Clearing screen");
    }
}
