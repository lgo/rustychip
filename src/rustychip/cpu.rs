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
    key: [bool; 16]
}

impl Cpu {

    pub fn new() -> Cpu {
        let cpu: Cpu = Cpu {
            drawFlag: false,
            V: [0; 16],
            gfx: [0; 64*32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            key: [false; 16],

            pc: 0x200, // Program counter starts at 0x200
            I: 0, // Reset index register
            sp: 0, // Reset stack pointer
            memory: [0; 4096]
        };
        // load in Util::fontset
        for i in 0..80 {
            cpu.memory[i] = Util::fontset[i];
        }
        return cpu;
    }

    pub fn loadGame (&mut self, buffer: &[u8]) {
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub fn emulateCycle (&mut self) {
        // fetch opcode
        let opcode_bits: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        let opcode: Opcode = Opcode::new(opcode_bits);

        executeOpcode(&opcode, self);

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
