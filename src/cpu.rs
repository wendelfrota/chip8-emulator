use std::fs::File;
use std::io;
use std::io::Read;
use crate::opcode::Opcode;
use crate::constants::{CHIP8_WIDTH, CHIP8_HEIGHT};

const PROGRAM_START: u16 = 0x200;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const MAX_PROGRAM_SIZE: usize = MEMORY_SIZE - PROGRAM_START as usize;

#[derive(Clone)]
pub struct CPU {
    memory: [u8; MEMORY_SIZE],
    v: [u8; NUM_REGISTERS],
    i: u16,
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub display: [bool; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; MEMORY_SIZE],
            v: [0; NUM_REGISTERS],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
        }
    }

    pub fn load_to_memory(&mut self, filename: &str) -> Result<(), io::Error> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        if buffer.len() > MAX_PROGRAM_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File too large to read",
            ));
        }
        self.memory[PROGRAM_START as usize..(PROGRAM_START as usize + buffer.len())].copy_from_slice(&buffer);

        Ok(())
    }

    pub fn execute_cycle(&mut self) -> Result<(), String> {
        let opcode = self.fetch_opcode();
        let decoded_opcode = self.decode_opcode(opcode);
        self.pc += 2;
        self.execute_opcode(decoded_opcode)
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
            0x3000 => Opcode::SE_Vx_byte(x, kk),
            0x4000 => Opcode::SNE_Vx_byte(x, kk),
            0x5000 if n == 0 => Opcode::SE_Vx_Vy(x, y),
            0x6000 => Opcode::LD_Vx_byte(x, kk),
            0x7000 => Opcode::ADD_Vx_byte(x, kk),
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
                _ => Opcode::INVALID(opcode),
            },
            0x9000 if n == 0 => Opcode::SNE_Vx_Vy(x, y),
            0xA000 => Opcode::LD_I_addr(nnn),
            0xB000 => Opcode::JP_V0_addr(nnn),
            0xC000 => Opcode::RND_Vx_byte(x, kk),
            0xD000 => Opcode::DRW_Vx_Vy_nibble(x, y, n),
            0xE000 => match kk {
                0x9E => Opcode::SKP_Vx(x),
                0xA1 => Opcode::SKNP_Vx(x),
                _ => Opcode::INVALID(opcode),
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
                _ => Opcode::INVALID(opcode),
            },
            _ => Opcode::INVALID(opcode),
        }
    }

    pub fn execute_opcode(&mut self, opcode: Opcode) -> Result<(), String> {
        match opcode {
            Opcode::CLS => self.cls(),
            Opcode::RET => self.ret(),
            Opcode::SYS(_) => Ok(()),
            Opcode::JP(addr) => self.jp(addr),
            Opcode::CALL(nnn) => self.call(nnn),
            Opcode::SE_Vx_byte(x, kk) => self.se_vx_byte(x, kk),
            Opcode::SNE_Vx_byte(x, kk) => self.sne_vx_byte(x, kk),
            Opcode::INVALID(op) => Err(format!("Invalid opcode: 0x{:04X}", op)),
            _ => Err("Invalid opcode".to_string())
        }
    }

    fn cls(&mut self) -> Result<(), String> {
        self.display = [false; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize];
        Ok(())
    }

    fn ret(&mut self) -> Result<(), String> {
        if self.sp == 0 {
            return Err("Stack underflow".to_string());
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        Ok(())
    }

    fn sys(&mut self, addr: u16) {
        if addr > 0xFFF {
            panic!("Invalid address for SYS: {:X}", addr);
        }
        if self.sp == 16 {
            panic!("Stack overflow!");
        }
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn jp(&mut self, addr: u16) -> Result<(), String> {
        if addr >= MEMORY_SIZE as u16 {
            return Err(format!("Invalid address for JP: 0x{:04X}", addr));
        }
        self.pc = addr;
        Ok(())
    }

    fn call(&mut self, nnn: u16) -> Result<(), String> {
        if self.sp as usize == STACK_SIZE {
            return Err("Stack overflow".to_string());
        }
        if nnn >= MEMORY_SIZE as u16 {
            return Err(format!("Invalid address for CALL: 0x{:04X}", nnn));
        }
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
        Ok(())
    }

    fn se_vx_byte(&mut self, x: u8, kk: u8) -> Result<(), String> {
        if x as usize >= NUM_REGISTERS {
            return Err(format!("Invalid register index: {}", x));
        }
        if self.v[x as usize] == kk {
            self.pc += 2;
        }
        Ok(())
    }

    fn sne_vx_byte(&mut self, x: u8, kk: u8) -> Result<(), String> {
        if x as usize >= NUM_REGISTERS {
            return Err(format!("Invalid register index: {}", x));
        }
        if self.v[x as usize] != kk {
            self.pc += 2;
        }
        Ok(())
    }
}
