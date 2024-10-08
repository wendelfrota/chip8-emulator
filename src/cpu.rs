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
        let opcode = self.fetch_opcode();
        let opcode = self.decode_opcode(opcode);

        self.execute_opcode(opcode);
    }

    fn fetch_opcode(&self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        (high_byte << 8) | low_byte
    }

    fn decode_opcode(&self, opcode: u16) -> Opcode {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;


        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Opcode::CLS,
                0x00EE => Opcode::RET,
                _ => Opcode::SYS(nnn),
            },
            0x1000 => Opcode::JP(nnn),
            0x2000 => Opcode::CALL(nnn),
            0x3000 => Opcode::SE_Vx_byte(x),
            0x4000 => Opcode::SNE_Vx_byte(x),
            0x5000 if n == 0 => Opcode::SE_Vx_Vy(x, y),
            0x6000 => Opcode::LD_Vx_byte(x),
            0x7000 => Opcode::ADD_Vx_byte(x),
            0x8000 => match n {
                0x0 => Opcode::LD_Vx_Vy(x, y),
                0x1 => Opcode::OR_Vx_Vy(x, y),
                0x2 => Opcode::AND_Vx_Vy(x, y),
                0x3 => Opcode::XOR_Vx_Vy(x, y),
                0x4 => Opcode::ADD_Vx_Vy(x, y),
                0x5 => Opcode::SUB_Vx_Vy(x, y),
                0x6 => Opcode::SHR_Vx(x),
                0x7 => Opcode::SUBN_Vx_Vy(x, y),
                0xE => Opcode::SHL_Vx(x),
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            0x9000 if n == 0 => Opcode::SNE_Vx_Vy(x, y),
            0xA000 => Opcode::LD_I_addr(nnn),
            0xB000 => Opcode::JP_V0_addr(nnn),
            0xC000 => Opcode::RND_Vx_byte(x),
            0xD000 => Opcode::DRW_Vx_Vy_nibble(x, y, n),
            0xE000 => match kk {
                0x9E => Opcode::SKP_Vx(x),
                0xA1 => Opcode::SKNP_Vx(x),
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            0xF000 => match kk {
                0x07 => Opcode::LD_Vx_DT(x),
                0x0A => Opcode::LD_Vx_K(x),
                0x15 => Opcode::LD_DT_Vx(x),
                0x18 => Opcode::LD_ST_Vx(x),
                0x1E => Opcode::ADD_I_Vx(x),
                0x29 => Opcode::LD_F_Vx(x),
                0x33 => Opcode::LD_B_Vx(x),
                0x55 => Opcode::LD_I_Vx(x),
                0x65 => Opcode::LD_Vx_I(x),
                _ => panic!("Unknown opcode: {:X}", opcode),
            },
            _ => panic!("Unknown opcode: {:X}", opcode),
        }
    }

    pub fn execute_opcode(&mut self, opcode: Opcode) {
       println!("Executing CPU instruction");
    }

    fn clear_display(&mut self) {
        self.display = [false; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize];
    }
}
