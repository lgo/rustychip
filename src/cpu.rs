use instructions::opcode_execute;
use opcode::Opcode;
use keypad::Keypad;
use display::Display;
use util as util;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: usize,
    pub pc: usize,
    pub gfx: [u8; 64 * 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    pub keypad: Keypad,
    pub display: Display
}

impl Cpu {

    pub fn new() -> Cpu {
        let mut cpu: Cpu = Cpu {
            v: [0; 16],
            gfx: [0; 64*32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],

            pc: 0x200, // Program counter starts at 0x200
            i: 0, // Reset index register
            sp: 0, // Reset stack pointer
            memory: [0; 4096],

            display: Display::new(),
            keypad: Keypad::new()
        };
        // load in Util::fontset
        for i in 0..80 {
            cpu.memory[i] = util::FONTSET[i];
        }
        return cpu;
    }

    pub fn emulate_cycle (&mut self) {
        // fetch opcode
        let opcode_bits: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        let opcode: Opcode = Opcode::new(opcode_bits);

        util::debug_cycle(self, &opcode);

        opcode_execute(self, &opcode);

        self.pc += 2;

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 0 {
                println!("BUZZ");
           }
           self.sound_timer -= 1;
        }
    }
}
