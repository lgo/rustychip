type Instruction = u16;

struct Opcode {
    code: Instruction,
    X: Instruction,
    Y: Instruction,
    address: Instruction,
    const8bit: Instruction,
    const4bit: Instruction
}

impl Opcode {
    fn new(Instruction: code) {
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
