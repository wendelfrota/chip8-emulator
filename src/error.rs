#[derive(Debug)]
pub enum Chip8Error {
    InvalidOpcode(u16),
    StackOverflow,
    StackUnderflow,
    InvalidMemoryAccess(usize),
    InvalidRegister(u8),
}
