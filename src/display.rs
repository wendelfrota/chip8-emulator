use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{WindowBuilder, Window};

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const SCALE_FACTOR: u32 = 10;


pub struct Display {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels
}

impl Display {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
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
            SurfaceTexture::new(CHIP8_WIDTH, CHIP8_HEIGHT, &window)
        ).unwrap();

        Display {
            event_loop,
            window,
            pixels
        }
    }

    pub fn run(mut self) -> Result<(), String> {
        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                },
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.pixels.resize_surface(size.width, size.height);
                    self.pixels.resize_buffer(size.width, size.height);
                }
                Event::RedrawRequested(_) => {
                    let frame = self.pixels.get_frame();

                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let color = if i % 4 == 0 { 0xFF } else { 0x00 };
                        pixel.copy_from_slice(&[color, color, color, 0xFF]);
                    }
                    if let Err(e) = self.pixels.render() {
                        eprintln!("Render error: {:?}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });

        Ok(())
    }

    pub fn clear(&mut self) {
        let frame = self.pixels.get_frame();

        let color = 0x00;
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[color, color, color, 0xFF]);
        }
    }
}
