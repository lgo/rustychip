use rustychip::opcode::Opcode as Opcode;
use rustychip::cpu::Cpu as Cpu;

pub fn executeOpcode(opcode: &Opcode, cpu: &Cpu) {
    // execute opcode
    match opcode.code & 0xF000 {
        0x0000 => {
            match opcode.code {
                0x00E0 => {
                    // CLS
                    // clear screen
                    cpu.gfx = [0; 2048];
                },
                0x00EE => {
                    // RET
                    // returns from subroutine
                    cpu.sp -= 1;
                },
                _ => {
                    // SYS addr
                    // call RCA 1802 at addr
                    // ??
                },
            }
        },
        0x1000 => {
            // JP addr
            // jumps to addr
            cpu.pc = opcode.address;
        },
        0x2000 => {
            // CALL addr
            // calls subroutine at addr
            cpu.stack[cpu.sp as usize] = opcode.address;
            cpu.sp += 1;
        },
        0x3000 => {
            // SE Vx, byte
            // skips next instruction if VX eq NN
            if (cpu.V[opcode.X as usize] == opcode.const8bit) {
                cpu.pc += 2;
            }
        },
        0x4000 => {
            // SNE Vx, byte
            // skips next instruction if VX neq MM
            if (cpu.V[opcode.X as usize] != opcode.const8bit) {
                cpu.pc += 2;
            }
        },
        0x5000 => {
            // SE Vx, Vy
            // skips next instruction if VX eq VY
            if (cpu.V[opcode.X as usize] != opcode.const8bit) {
                cpu.pc += 2;
            }
        },
        0x6000 => {
            // LD Vx, byte
            // sets VX to NN
            cpu.V[opcode.X as usize] = opcode.const8bit;
        },
        0x7000 => {
            // ADD Vx, byte
            // incr VX by NN
            cpu.V[opcode.X as usize] += opcode.const8bit;
        },
        0x8000 => {
            match opcode.code & 0x000F {
                0x0000 => {
                    // LD Vx, Vy
                    // set VX to VY
                    cpu.V[opcode.X as usize] += cpu.V[opcode.Y as usize];
                },
                0x0001 => {
                    // OR Vx, Vy
                    // sets VX to VX or VY
                    cpu.V[opcode.X as usize] |= cpu.V[opcode.Y as usize];
                },
                0x0002 => {
                    // AND Vx, Vy
                    // sets VX to VX and VY
                    cpu.V[opcode.X as usize] &= cpu.V[opcode.Y as usize];
                },
                0x0003 => {
                    // XOR Vx, Vy
                    // sets VX to VX and VY
                    cpu.V[opcode.X as usize] ^= cpu.V[opcode.Y as usize];
                },
                0x0004 => {
                    // ADD Vx, Vy
                    // incr VX by VY, if VF = 1 if carry else 0
                    if (cpu.V[opcode.Y as usize] < (0xFF - cpu.V[opcode.X as usize])) {
                        cpu.V[0xF] = 1;
                    }
                    else {
                        cpu.V[0xF] = 0;
                    };

                    cpu.V[opcode.X as usize] -= cpu.V[opcode.Y as usize];
                },
                0x0005 => {
                    // SUB Vx, Vy
                    // sub VX by VY, if VF = 0 if borrow else 1
                    if (cpu.V[opcode.Y as usize] > cpu.V[opcode.X as usize]) {
                        cpu.V[0xF] = 1;
                    }
                    else {
                        cpu.V[0xF] = 0;
                    }

                    cpu.V[opcode.X as usize] -= cpu.V[opcode.Y as usize];
                },
                0x0006 => {
                    // SHR Vx {, Vy}
                    // right ship VX by 1. Set VF to the least significant bit before shifting
                    cpu.V[0xF] = cpu.V[opcode.X as usize] & 0x0001;
                    cpu.V[opcode.X as usize] >> 1;
                },
                0x0007 => {
                    // SUBN Vx, Vy
                    // sub VX by VY, if VF = 0 if borrow else 1
                    if (cpu.V[opcode.X as usize] > cpu.V[opcode.Y as usize]) {
                        cpu.V[0xF] = 1;
                    }
                    else {
                        cpu.V[0xF] = 0;
                    }

                    cpu.V[opcode.X as usize] = cpu.V[opcode.Y as usize] - cpu.V[opcode.X as usize];
                },
                0x000E => {
                    // SHL Vx {, Vy}
                    // Set Vx = Vx SHL 1.
                    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
                    // Then Vx is multiplied by 2.
                    // TODO
                }
                _ => {},
            }
        },
        0x9000 => {
            // SNE Vx, Vy
            // skips the next instruction if VX neq Vy
            if cpu.V[opcode.X as usize] != cpu.V[opcode.Y as usize] {
                cpu.pc += 2;
            }
        },
        0xA000 => {
            // LD I, addr
            // sets I to addr
            cpu.I = opcode.address;
        },
        0xB000 => {
            // JP V0, addr
            // jumps to address + V0
            cpu.pc = opcode.address as u16 + cpu.V[0x0] as u16;
        },
        0xC000 => {
            // RND Vx, byte
            // sets VX to a random number, masked by NN
            let rand = 0;
            cpu.V[opcode.X as usize] = rand & opcode.const8bit;
        },
        0xD000 => {
            // DRW Vx, Vy, nibble
            // Sprites stored in memory at location in index register (I), maximum 8bits wide.
            // Wraps around the screen. If when drawn, clears a pixel,
            // register VF is set to 1 otherwise it is zero.
            // All drawing is XOR drawing (i.e. it toggles the screen pixels)
            let x: u8 = cpu.V[opcode.X as usize];
            let y: u8 = cpu.V[opcode.Y as usize];
            let height: u8 = opcode.const4bit as u8;
            let pixel: u8;
            cpu.V[0xF] = 0;
            for yline in 0..height {
                pixel = cpu.memory[(cpu.I + yline as u16) as usize];
                for xline in 0..8 {
                    if((pixel & (0x80 >> xline)) != 0) {
                        if(cpu.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1) {
                            cpu.V[0xF] = 1;
                        }
                        cpu.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                    }
                }
            }
            cpu.drawFlag = true;
        },
        0xE000 => {
            match opcode.code & 0x00FF {
                0x009E => {
                    // SKP Vx
                    // Skips the next instruction if the key stored in VX is pressed.
                    if (cpu.key[cpu.V[opcode.X as usize] as usize]) {
                        cpu.pc += 2;
                    }
                },
                0x00A1 => {
                    // SKNP Vx
                    // Skips the next instruction if the key stored in VX isn't pressed.
                    if (!cpu.key[cpu.V[opcode.X as usize] as usize]) {
                        cpu.pc += 2;
                    }
                },
                _ => {},
            }
        },
        0xF000 => {
            match opcode.code & 0x00FF {
                0x0007 => {
                    // LD Vx, DT
                    // Sets VX to the value of the delay timer.
                    cpu.V[opcode.X as usize] = cpu.delay_timer;
                },
                0x000A => {
                    // LD Vx, K
                    // A key press is awaited, and then stored in VX.
                    // wait?
                    // cpu.V[opcode.X as usize] = 0x0;
                },
                0x0015 => {
                    // LD DT, Vx
                    // Sets the delay timer to VX.
                    cpu.delay_timer = cpu.V[opcode.X as usize];
                },
                0x0018 => {
                    // LD ST, Vx
                    // Sets the sound timer to VX.
                    cpu.sound_timer = cpu.V[opcode.X as usize];
                },
                0x001E => {
                    // ADD I, Vx
                    // adds VX to I

                    // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
                    // This is undocumented feature of the CHIP-8 and used by Spacefight 2091! game.
                    if (cpu.I + (cpu.V[opcode.X as usize] as u16) > 0xFFF) {
                        cpu.V[0xF] = 1;
                    }
                    else {
                        cpu.V[0xF] = 0;
                    }

                    cpu.I += cpu.V[opcode.X as usize] as u16;
                },
                0x0029 => {
                    // LD F, Vx
                    // Sets I to the location of the sprite for the character in VX.
                    // Characters 0-F (in hexadecimal) are represented by a 4x5 font.

                },
                0x0033 => {
                    // LD B, Vx
                    // Stores the Binary-coded decimal representation of VX,
                    // with the most significant of three digits at the address in I,
                    // the middle digit at I plus 1, and the least significant digit at I plus 2.
                    // (In other words, take the decimal representation of VX, place the hundreds
                    // digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
                },
                0x0055 => {
                    // LD [I], Vx
                    // Stores V0 to VX in memory starting at address I.
                    for i in 0..cpu.V[opcode.X as usize] {
                        cpu.memory[(cpu.I + (i as u16)) as usize] = cpu.V[i as usize];
                    }
                },
                0x0065 => {
                    // LD Vx, [I]
                    // Fills V0 to VX with values from memory starting at address I.
                    for i in 0..cpu.V[opcode.X as usize] {
                        cpu.V[i  as usize] = cpu.memory[(cpu.I + (i as u16)) as usize];
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }
}
