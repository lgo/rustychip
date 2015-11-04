struct Opcode {
    code: u16,
    X: u16,
    Y: u16,
    address: u16,
    const8bit: u16,
    const4bit: u16
}

impl Opcode {
    fn new(code: u16) {
        return Opcode {
            code: code,
            X: code & 0x0F00 << 8,
            Y: code & 0x00F0 << 4,
            address: code & 0x0FFF,
            const8bit: code & 0x000F,
            const4bit: code & 0x00FF
        }
    }
}
