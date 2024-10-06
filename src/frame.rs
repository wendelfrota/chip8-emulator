use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use crate::display::Display;

pub struct Frame {
    display: Display,
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels
}

impl Frame {
    pub fn new(size: Option<(u32, u32)>, scale_factor: Option<i32>) -> Self {
        let display = Display::new(size);
        let scale_factor = scale_factor.unwrap_or_else(|| 10);

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Chip8 Emulator")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                display.pixels[0].len() as i32 * scale_factor,
                display.pixels.len() as i32 * scale_factor,
            ))
            .build(&event_loop)
            .unwrap();

        let pixels = Pixels::new(
            display.size.0,
            display.size.1,
            SurfaceTexture::new(display.size.0, display.size.1, &window)
        ).unwrap();

        Frame {
            display,
            event_loop,
            window,
            pixels
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.pixels.resize_surface(size.width, size.height);
                    self.pixels.resize_buffer(size.width, size.height);
                    self.display.size = (size.width, size.height);
                }
                Event::RedrawRequested(_) => {
                    let frame = self.pixels.get_frame();

                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let color = if i % 4 == 0 { 0xFF } else { 0x00 };
                        pixel.copy_from_slice(&[color, color, color, 0xFF]);
                    }
                    if let Err(e) = self.pixels.render() {
                        eprintln!("Render error: {:?}", e);
                    }
                }
                _ => (),
            }
        });
    }

    pub fn clear(&mut self) {
        let frame = self.pixels.get_frame();

        let color = 0x00;
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[color, color, color, 0xFF]);
        }
    }
}
