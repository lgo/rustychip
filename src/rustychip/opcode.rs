struct Opcode {
    code: u16,
    X: u16,
    Y: u16,
    address: u16,
    const8bit: u8,
    const4bit: u8
}

impl Opcode {
    fn new(code: u16) -> Opcode {
        return Opcode {
            code: code,
            X: code & 0x0F00 << 8,
            Y: code & 0x00F0 << 4,
            address: code & 0x0FFF,
            const8bit: code as u8 & 0x0F,
            const4bit: code as u8 & 0xFF
        }
    }
}
