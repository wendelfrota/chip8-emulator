use crate::cpu::CPU;
use crate::display::Display;

pub struct Emulator {
    cpu: CPU,
    display: Display
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            display: Display::new(),
        }
    }

    pub fn start(self) {
        self.display.run();
    }
}
