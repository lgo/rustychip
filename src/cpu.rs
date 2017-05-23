use opcode::Opcode;
use keypad::Keypad;
use display::Display;
use util;

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
    pub display: Display,
}

fn not_implemented(op: usize, pc: usize) {
    println!("Not implemented:: op: {:x}, pc: {:x}", op, pc);
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu: Cpu = Cpu {
            v: [0; 16],
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],

            pc: 0x200, // Program counter starts at 0x200
            i: 0, // Reset index register
            sp: 0, // Reset stack pointer
            memory: [0; 4096],

            display: Display::new(),
            keypad: Keypad::new(),
        };
        // load in Util::fontset
        for i in 0..80 {
            cpu.memory[i] = util::FONTSET[i];
        }
        return cpu;
    }

    pub fn emulate_cycle(&mut self) {
        // fetch opcode
        let opcode_bits: u16 = (self.memory[self.pc as usize] as u16) << 8 |
                               self.memory[(self.pc + 1) as usize] as u16;
        let opcode: Opcode = Opcode::new(opcode_bits);

        util::debug_cycle(self, &opcode);

        self.pc += 2;

        self.execute(&opcode);

        // ??
        // self.pc += 2;

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

    pub fn execute(&mut self, opcode: &Opcode) {
        match opcode.code & 0xf000 {
            0x0000 => self.execute_clear_return(opcode), // 0nnn - SYS  nnn
            0x1000 => op_1xxx(self, opcode),
            0x2000 => op_2xxx(self, opcode),
            0x3000 => op_3xxx(self, opcode),
            0x4000 => op_4xxx(self, opcode),
            0x5000 => op_5xxx(self, opcode),
            0x6000 => op_6xxx(self, opcode),
            0x7000 => op_7xxx(self, opcode),
            0x8000 => op_8xxx(self, opcode),
            0x9000 => op_9xxx(self, opcode),
            0xA000 => op_Axxx(self, opcode),
            0xB000 => op_Bxxx(self, opcode),
            0xC000 => op_Cxxx(self, opcode),
            0xD000 => op_Dxxx(self, opcode),
            0xE000 => op_Exxx(self, opcode),
            0xF000 => op_Fxxx(self, opcode),
            _ => not_implemented(opcode.code as usize, self.pc),
        }
    }
    pub fn execute_clear_return(&mut self, opcode: &Opcode) {
        match opcode.code {
            0x00E0 => {
                // CLS
                // clear screen
                self.display.clear();
            }
            0x00EE => self.return_from_subroutine(),
            _ => not_implemented(opcode.code as usize, self.pc),
        }
    }

    pub fn return_from_subroutine(&mut self) {
        // 00EE - RTS
        // Return from subroutine. Pop the current value in the stack pointer
        // off of the stack, and set the program counter to the value popped.
        // self.registers['sp'] -= 1
        // self.registers['pc'] = self.memory[self.registers['sp']] << 8
        // self.registers['sp'] -= 1
        // self.registers['pc'] += self.memory[self.registers['sp']]
        self.sp -= 1;
    }

    //     0x0: self.clear_return,                  # 0nnn - SYS  nnn
    //          0x1: self.jump_to_address,               # 1nnn - JUMP nnn
    //          0x2: self.jump_to_subroutine,            # 2nnn - CALL nnn
    //          0x3: self.skip_if_reg_equal_val,         # 3snn - SKE  Vs, nn
    //          0x4: self.skip_if_reg_not_equal_val,     # 4snn - SKNE Vs, nn
    //          0x5: self.skip_if_reg_equal_reg,         # 5st0 - SKE  Vs, Vt
    //          0x6: self.move_value_to_reg,             # 6snn - LOAD Vs, nn
    //          0x7: self.add_value_to_reg,              # 7snn - ADD  Vs, nn
    //          0x8: self.execute_logical_instruction,   # see subfunctions below
    //          0x9: self.skip_if_reg_not_equal_reg,     # 9st0 - SKNE Vs, Vt
    //          0xA: self.load_index_reg_with_value,     # Annn - LOAD I, nnn
    //          0xB: self.jump_to_index_plus_value,      # Bnnn - JUMP [I] + nnn
    //          0xC: self.generate_random_number,        # Ctnn - RAND Vt, nn
    //          0xD: self.draw_sprite,                   # Dstn - DRAW Vs, Vy, n
    //          0xE: self.keyboard_routines,             # see subfunctions below
    //          0xF: self.misc_routines,                 # see subfunctions below
}
