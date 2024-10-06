use crate::frame::Frame;
use crate::opcode::Opcode;

pub struct CPU {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
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
        }
    }

    pub fn start_frame(&mut self) {
        let frame = Frame::new(None, None);
        frame.run()
    }

    pub fn execute_opcode(opcode: Opcode) {
        match opcode {
            Opcode::CLS => {},
            Opcode::RET => {},
            Opcode::SYS(_) => {},
            Opcode::JP(_) => {},
            Opcode::CALL(_) => {},
            Opcode::SE_Vx_byte(_) => {},
            Opcode::SNE_Vx_byte(_) => {},
            Opcode::SE_Vx_Vy(_, _) => {},
            Opcode::LD_Vx_byte(_) => {},
            Opcode::ADD_Vx_byte(_) => {},
            Opcode::LD_Vx_Vy(_, _) => {},
            Opcode::OR_Vx_Vy(_, _) => {},
            Opcode::AND_Vx_Vy(_, _) => {},
            Opcode::XOR_Vx_Vy(_, _) => {},
            Opcode::ADD_Vx_Vy(_, _) => {},
            Opcode::SUB_Vx_Vy(_, _) => {},
            Opcode::SHR_Vx(_) => {},
            Opcode::SUBN_Vx_Vy(_, _) => {},
            Opcode::SHL_Vx(_) => {},
            Opcode::SNE_Vx_Vy(_, _) => {},
            Opcode::LD_I_addr(_) => {},
            Opcode::JP_V0_addr(_) => {},
            Opcode::RND_Vx_byte(_) => {},
            Opcode::DRW_Vx_Vy_nibble(_, _, _) => {},
            Opcode::SKP_Vx(_) => {},
            Opcode::SKNP_Vx(_) => {},
            Opcode::LD_Vx_DT(_) => {},
            Opcode::LD_Vx_K(_) => {},
            Opcode::LD_DT_Vx(_) => {},
            Opcode::LD_ST_Vx(_) => {},
            Opcode::ADD_I_Vx(_) => {},
            Opcode::LD_F_Vx(_) => {},
            Opcode::LD_B_Vx(_) => {},
            Opcode::LD_I_Vx(_) => {},
            Opcode::LD_Vx_I(_) => {},
        }
    }
}
