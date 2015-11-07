pub struct Opcode {
    pub code: u16,
    pub x: usize,
    pub y: usize,
    pub address: usize,
    pub const8bit: u8,
    pub const4bit: u8
}

impl Opcode {
    pub fn new(code: u16) -> Opcode {
        return Opcode {
            code: code,
            x: ((code & 0x0F00) >> 8) as usize,
            y: ((code & 0x00F0) >> 4) as usize,
            address: (code & 0x0FFF) as usize,
            const8bit: code as u8 & 0x0F,
            const4bit: code as u8 & 0xFF
        }
    }
}
