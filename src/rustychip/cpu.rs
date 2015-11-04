use rustychip::instructions::executeOpcode as executeOpcode;
use rustychip::opcode::Opcode as Opcode;
use rustychip::util as Util;

pub struct Cpu {
    drawFlag: bool,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16]
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            pc: 0x200, // Program counter starts at 0x200
            opcode: 0, // Reset current opcode
            I: 0, // Reset index register
            sp: 0, // Reset stack pointer
            memory: [..Util::fontset, 0] // load in font_set
        }
    }

    pub fn loadGame (&mut self, buffer: [u8]) {
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub fn emulateCycle (&mut self) {
        // fetch opcode
        let opcode_bits: u16 = self.memory[self.pc] << 8 | self.memory[self.pc + 1];
        let opcode: Opcode = Opcode::new(opcode_bits);

        executeOpcode(opcode, self);

        self.pc += 2;

        // update timers
        if (self.delay_timer > 0) {
            self.delay_timer -= 1;
        }
        if (self.sound_timer > 0) {
            self.delay_timer -= 1;
            if (self.sound_timer == 0) {
                // buzz
           }
        }
    }
}
