pub enum Opcode {
    CLS,                          // 00E0
    RET,                          // 00EE
    SYS(u16),                     // 0nnn
    JP(u16),                      // 1nnn
    CALL(u16),                    // 2nnn
    SE_Vx_byte(u8),               // 3xkk
    SNE_Vx_byte(u8),              // 4xkk
    SE_Vx_Vy(u8, u8),             // 5xy0
    LD_Vx_byte(u8),               // 6xkk
    ADD_Vx_byte(u8),              // 7xkk
    LD_Vx_Vy(u8, u8),             // 8xy0
    OR_Vx_Vy(u8, u8),             // 8xy1
    AND_Vx_Vy(u8, u8),            // 8xy2
    XOR_Vx_Vy(u8, u8),            // 8xy3
    ADD_Vx_Vy(u8, u8),            // 8xy4
    SUB_Vx_Vy(u8, u8),            // 8xy5
    SHR_Vx(u8),                   // 8xy6
    SUBN_Vx_Vy(u8, u8),           // 8xy7
    SHL_Vx(u8),                   // 8xyE
    SNE_Vx_Vy(u8, u8),            // 9xy0
    LD_I_addr(u16),               // Annn
    JP_V0_addr(u16),              // Bnnn
    RND_Vx_byte(u8),              // Cxkk
    DRW_Vx_Vy_nibble(u8, u8, u8), // Dxyn
    SKP_Vx(u8),                   // Ex9E
    SKNP_Vx(u8),                  // ExA1
    LD_Vx_DT(u8),                 // Fx07
    LD_Vx_K(u8),                  // Fx0A
    LD_DT_Vx(u8),                 // Fx15
    LD_ST_Vx(u8),                 // Fx18
    ADD_I_Vx(u8),                 // Fx1E
    LD_F_Vx(u8),                  // Fx29
    LD_B_Vx(u8),                  // Fx33
    LD_I_Vx(u8),                  // Fx55
    LD_Vx_I(u8),                  // Fx65
}
