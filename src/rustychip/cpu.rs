use rustychip::instructions::executeOpcode as executeOpcode;
use rustychip::opcode::Opcode as Opcode;
use rustychip::consts as Consts;
use rustychip::mem::Mem as Mem;

pub struct Cpu {
    drawFlag: bool,
    memory: Mem<u8>, // 4096
    V: Mem<u8>, // 16
    I: u16,
    pc: u16,
    gfx: Mem<u8>, // 64 * 32
    delay_timer: u8,
    sound_timer: u8,
    stack: Mem<u16>, // 16
    sp: u16,
    key: Mem<u8> // 16
}

impl Cpu {

    pub fn new() -> Cpu {
        let memory: Mem<u8>;
        for i in 0..Consts::font_set.length() {
            memory[i] = Consts::font_set;
        }
        for i in Consts::font_set.length()..4096 {
            memory[i] = 0;
        }

        Cpu {
            pc: 0x200, // Program counter starts at 0x200
            memory: memory,
            opcode: 0, // Reset current opcode
            I: 0, // Reset index register
            sp: 0, // Reset stack pointer
            memory: [..Consts::fontset, 0] // load in font_set
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
