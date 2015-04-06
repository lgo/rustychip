type byte = u8;

static chip8_fontset: u8[80] =
{
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
};


struct Chip8 {
    drawFlag: bool
    memory: u8[4096],
    V: u8[16],
    I: u16,
    pc: u16,
    gfx: u8[64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: u16[16],
    sp: u16,
    key: u8[16]
}

impl Chip8 {

    fn new() -> Chip8 {
        self.pc = 0x200; // Program counter starts at 0x200
        self.opcode = 0; // Reset current opcode
        self.I = 0; // Reset index register
        self.sp = 0; // Reset stack pointer

        // Clear display
        // Clear stack
        // Clear registers V0-VF
        // Clear memory

        // Load fontset
        for(int i = 0; i < 80; ++i)
            memory[i] = chip8_fontset[i];
        }
    }

    fn loadGame () {
        for(int i = 0; i < bufferSize; ++i) {
            memory[i + 512] = buffer[i];
        }
    }

    fn emulateCycle () {
        // fetch opcode
        u16 opcode_bits = self.memory[pc] << 8 |
                          self.memory[pc + 1];
        Opcode opcode = Opcode::new(opcode_bits);

        // execute opcode
        match opcode.code & 0xF000 {
            0x0000 => {
                match opcode.code {
                    0x00E0 => {
                        // clear screen
                        self.gfx = [0];
                    },
                    0x00EE => {
                        // returns from subroutine
                        self.sp--;
                    },
                    _ => {
                        // call RCA 1802 at addr
                        // ??
                    },
                }
            },
            0x1000 => {
                // jumps to addr
                self.pc = opcode.address
            },
            0x2000 => {
                // calls subroutine at addr
                self.stack[self.sp] = self.addr
                self.sp++;
            },
            0x3000 => {
                // skips next instruction if VX eq NN
                if (self.V[opcode.X] == opcode.const8bit) {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // skips next instruction if VX neq MM
                if (self.V[opcode.X] != opcode.const8bit) {
                    self.pc += 2;
                }
            },
            0x5000 => {
                // skips next instruction if VX eq VY
                if (self.V[opcode.X] != opcode.const8bit) {
                    self.pc += 2;
                }
            },
            0x6000 => {
                // sets VX to NN
                self.V[opcode.X] = opcode.const8bit;
            },
            0x7000 => {
                // incr VX by NN
                self.V[opcode.X] += opcode.const8bit;
            },
            0x8000 => {
                match opcode.code & 0x000F {
                    0x0000 => {
                        // set VX to VY
                        self.V[opcode.X] += self.V[opcode.Y];
                    },
                    0x0001 => {
                        // sets VX to VX or VY
                        self.V[opcode.X] |= self.V[opcode.Y];
                    },
                    0x0002 => {
                        // sets VX to VX and VY
                        self.V[opcode.X] &= self.V[opcode.Y];
                    },
                    0x0003 => {
                        // sets VX to VX and VY
                        self.V[opcode.X] ^= self.V[opcode.Y];
                    },
                    0x0004 => {
                        // incr VX by VY, if VF = 1 if carry else 0
                        if (self.V[opcode.Y] < (0xFF - self.V[opcode.X])) {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        };

                        self.V[opcode.X] -= self.V[opcode.Y];
                    },
                    0x0005 => {
                        // sub VX by VY, if VF = 0 if borrow else 1
                        if (self.V[opcode.Y] > self.V[opcode.X]) {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }

                        self.V[opcode.X] -= self.V[opcode.Y];
                    },
                    0x0006 => {
                        // right ship VX by 1. Set VF to the least significant bit before shifting
                        self.V[0xF] = self.V[opcode.X] & 0x0001;
                        self.V[opcode.X] >> 1;
                    },
                    0x0007 => {
                        // sub VX by VY, if VF = 0 if borrow else 1
                        if (self.V[opcode.X] > self.V[opcode.Y]) {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }

                        self.V[opcode.X] = self.V[opcode.Y] - self.V[opcode.X];
                    },
                    _ => {},
                }
            },
            0x9000 => {
                // skips the next instruction if VX neq Vy
                if self.V[opcode.X] != self.V[opcode.Y] {
                    pc += 2;
                }
            },
            0xA000 => {
                // sets I to addr
                self.I = opcode.address;
            },
            0xB000 => {
                // jumps to address + V0
                self.pc = opcode.address + self.V[0x0];
            },
            0xC000 => {
                // sets VX to a random number, masked by NN
                let rand = 0;
                self.V[opcode.X] = rand & opcode.const8bit;
            },
            0xD000 => {
                // Sprites stored in memory at location in index register (I), maximum 8bits wide.
                // Wraps around the screen. If when drawn, clears a pixel,
                // register VF is set to 1 otherwise it is zero.
                // All drawing is XOR drawing (i.e. it toggles the screen pixels)
                u8 x = self.V[opcode.X];
                u8 y = self.V[opcode.Y];
                u8 height = self.const4bit;
                u8 pixel;
                self.V[0xF] = 0;
                for (u8 yline = 0; yline < height; yline++) {
                    pixel = self.memory[self.I + yline];
                    for(int xline = 0; xline < 8; xline++) {
                        if((pixel & (0x80 >> xline)) != 0) {
                            if(self.gfx[(x + xline + ((y + yline) * 64))] == 1) {
                                self.V[0xF] = 1;
                            }
                            self.gfx[x + xline + ((y + yline) * 64)] ^= 1;
                        }
                    }
                }
                self.drawFlag = true;
            },
            0xE000 => {
                match opcode.code & 0x00FF {
                    0x009E => {
                        // Skips the next instruction if the key stored in VX is pressed.
                        if (self.key[self.V[opcode.X]]) {
                            pc += 2;
                        }
                    },
                    0x00A1 => {
                        // Skips the next instruction if the key stored in VX isn't pressed.
                        if (!self.key[self.V[opcode.X]]) {
                            pc += 2;
                        }
                    },
                    _ => {},
                }
            },
            0xF000 => {
                match opcode.code & 0x00FF {
                    0x0007 => {
                        // Sets VX to the value of the delay timer.
                        self.V[opcode.X] = self.delay_timer;
                    },
                    0x000A => {
                        // A key press is awaited, and then stored in VX.
                        // wait?
                        // self.V[opcode.X] = 0x0;
                    },
                    0x0015 => {
                        // Sets the delay timer to VX.
                        self.delay_timer = self.V[opcode.X];
                    },
                    0x0018 => {
                        // Sets the sound timer to VX.
                        self.sound_timer = self.V[opcode.X];
                    },
                    0x001E => {
                        // adds VX to I

                        // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
                        // This is undocumented feature of the CHIP-8 and used by Spacefight 2091! game.
                        if (self.I + self.V[opcode.X] > 0xFFF) {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }

                        self.I += self.V[opcode.X];
                    },
                    0x0029 => {
                        // Sets I to the location of the sprite for the character in VX.
                        // Characters 0-F (in hexadecimal) are represented by a 4x5 font.

                    },
                    0x0033 => {
                        // Stores the Binary-coded decimal representation of VX,
                        // with the most significant of three digits at the address in I,
                        // the middle digit at I plus 1, and the least significant digit at I plus 2.
                        // (In other words, take the decimal representation of VX, place the hundreds
                        // digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
                    },
                    0x0055 => {
                        // Stores V0 to VX in memory starting at address I.
                        for i..self.V[opcode.X] {
                            self.memory[I + i] = self.V[i];
                        }
                    },
                    0x0065 => {
                        // Fills V0 to VX with values from memory starting at address I.
                        for i..self.V[opcode.X] {
                            self.V[i] = self.memory[I + i];
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        pc += 2;

        // update timers
        if (delay_timer > 0) {
            delay_timer--;
        }
        if (sound_timer > 0) {
            delay_timer--;
            if (sound_timer == 0) {
                // buzz
           }
        }
    }

    fn setKeys () {

    }
}


impl Chip8 {

}
