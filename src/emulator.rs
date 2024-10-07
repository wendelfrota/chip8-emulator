use crate::cpu::CPU;
use crate::frame::Frame;

pub struct Emulator {
    cpu: CPU,
    frame: Frame
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            frame: Frame::new(None, None),
        }
    }

    pub fn start(self) {
        self.frame.run();
    }
}
