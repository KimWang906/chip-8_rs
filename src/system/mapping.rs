pub const CALL: u8 = 0x2;
pub const RET: (u8, u8, u8, u8) = (0, 0, 0xE, 0xE);
pub const NO_OP: (u8, u8, u8, u8) = (0, 0, 0, 0);
pub const ADD: u8 = 0x4;
pub const SUB: u8 = 0x5;
pub const MOV: u8 = 0x0;