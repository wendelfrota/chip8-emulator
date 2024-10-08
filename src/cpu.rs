use crate::opcode::Opcode;
use crate::constants::{CHIP8_WIDTH, CHIP8_HEIGHT};

pub struct CPU {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub display: [bool; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
        }
    }

    pub fn execute_cycle(&mut self) {
        println!("Executing CPU cycle");
    }

    fn fetch_opcode(&self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        (high_byte << 8) | low_byte
    }

    pub fn execute_opcode() {
       println!("Executing CPU instruction");
    }
}
