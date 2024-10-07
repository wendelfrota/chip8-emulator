use crate::cpu::CPU;
use crate::display::Display;

pub struct Emulator {
    cpu: CPU,
    display: Display,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            display: Display::new(),
        }
    }

    pub fn start(self) -> Result<(), String> {
        if let Err(e) = self.display.run() {
            return Err(format!("Failed to run display:\n\t {}", e));
        }

        Ok(())
    }
}
