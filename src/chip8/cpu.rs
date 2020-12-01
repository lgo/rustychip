use crate::chip8::instruction::Instruction;
use crate::chip8::timer::Timer;
use crate::interface::emulator::Clocked;
use crate::interface::serialization::Savable;

// Cpu is an emulation component for the Chip8 CPU.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Cpu {
  // Program counter for the CPU.
  program_counter: u16,
  // Address (i) register for the CPU.
  i_register: u16,
  // Stack pointer for the CPU.
  sp: u8,
  // Variable (v0-vf) registers for the CPU.
  v_registers: [u8; 16],
  // TODO(joey): document
  sound_timer: Timer,
  // TODO(joey): document
  delay_timer: Timer,
  // TODO(joey): document
  stack: [u16; 16],
  // TODO(joey): document
  memory: [u8; 4096],
}

impl Savable for Cpu {
  fn save(&self, fh: &mut dyn std::io::Write) -> std::io::Result<()> {
    self.v_registers.save(fh)?;
    self.i_register.save(fh)?;
    self.program_counter.save(fh)?;
    self.sp.save(fh)?;
    self.sound_timer.save(fh)?;
    self.delay_timer.save(fh)?;
    self.memory.save(fh)?;
    self.stack.save(fh)?;
    Ok(())
  }
  fn load(&mut self, fh: &mut dyn std::io::Read) -> std::io::Result<()> {
    self.v_registers.load(fh)?;
    self.i_register.load(fh)?;
    self.program_counter.load(fh)?;
    self.sp.load(fh)?;
    self.sound_timer.load(fh)?;
    self.delay_timer.load(fh)?;
    self.memory.load(fh)?;
    self.stack.load(fh)?;
    Ok(())
  }
}

impl Clocked for Cpu {
  fn clock(&mut self) {
    unimplemented!("TODO(joey): implement");
  }

  fn clock_rate(self) -> u32 {
    // The CHIP has a clock rate of approximately 500hz, but in practice
    // emulating (inaccurately) at 500hz is sufficient.
    //
    // TODO(joey): Clock rates for SuperCHIP should be 1000hz.
    500
  }
}

impl Default for Cpu {
  fn default() -> Self {
    Cpu::new()
  }
}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      v_registers: [0; 16],
      i_register: 0,
      program_counter: 0,
      delay_timer: Timer::new(),
      sound_timer: Timer::new(),
      memory: [0; 4096],
      stack: [0; 16],
      sp: 0,
    }
  }

  fn handle_instruction(&mut self, instr: Instruction) {
    use Instruction::*;
    match instr {
      Call { address } => unimplemented!("TODO(joey): implement"),
      DisplayClear() => unimplemented!("TODO(joey): implemenent"),
      FlowSubroutineReturn() => unimplemented!("TODO(joey): implemenent"),
      FlowJumpToAddress { address } => unimplemented!("TODO(joey): implement"),
      FlowSubroutineCall { address } => unimplemented!("TODO(joey): implement"),
      CondSkipIfEqualConst {
        x_register,
        constant,
      } => {
        if self.v_registers[x_register] == constant {
          self.program_counter += 1
        }
      }
      CondSkipIfNotEqualConst {
        x_register,
        constant,
      } => {
        if self.v_registers[x_register] != constant {
          self.program_counter += 1
        }
      }
      CondSkipIfEqualVar {
        x_register,
        y_register,
      } => {
        if self.v_registers[x_register] == self.v_registers[y_register] {
          self.program_counter += 1
        }
      }
      ConstSetVar {
        x_register,
        constant,
      } => self.v_registers[x_register] = constant,
      ConstIncrementVar {
        x_register,
        constant,
      } => self.v_registers[x_register] += constant,
      AssignVar {
        x_register,
        y_register,
      } => self.v_registers[x_register] = self.v_registers[y_register],
      BitwiseOrVar {
        x_register,
        y_register,
      } => self.v_registers[x_register] |= self.v_registers[y_register],
      BitwiseAndVar {
        x_register,
        y_register,
      } => self.v_registers[x_register] &= self.v_registers[y_register],
      BitwiseXorVar {
        x_register,
        y_register,
      } => self.v_registers[x_register] ^= self.v_registers[y_register],
      MathAddVar {
        x_register,
        y_register,
      } => {
        let result = self.v_registers[x_register] + self.v_registers[y_register];
        let has_overflow = (self.v_registers[x_register] < result);
        self.v_registers[0xF] = if has_overflow { 1 } else { 0 };
        self.v_registers[x_register] = result
      }
      MathSubVar {
        x_register,
        y_register,
      } => {
        let result = self.v_registers[x_register] - self.v_registers[y_register];
        let has_underflow = (self.v_registers[x_register] > result);
        self.v_registers[0xF] = if has_underflow { 0 } else { 1 };
        self.v_registers[x_register] = result
      }
      BitShiftRightVar {
        x_register,
        y_register,
      } => {
        let lsb = self.v_registers[x_register] & 0b0001;
        self.v_registers[0xF] = lsb;
        self.v_registers[x_register] >>= 1
      }
      MathReverseSubtractVar {
        x_register,
        y_register,
      } => {
        let result = self.v_registers[y_register] - self.v_registers[x_register];
        let has_underflow = (self.v_registers[y_register] > result);
        self.v_registers[0xF] = if has_underflow { 0 } else { 1 };
        self.v_registers[y_register] = result
      }
      BitShiftLeftVar {
        x_register,
        y_register,
      } => {
        let msb = self.v_registers[x_register] & 0b1000;
        self.v_registers[0xF] = msb;
        self.v_registers[x_register] <<= 1
      }
      CondSkipIfNotEqualVar {
        x_register,
        y_register,
      } => {
        if self.v_registers[x_register] != self.v_registers[y_register] {
          self.program_counter += 1
        }
      }
      MemorySetAddress { constant } => self.i_register = constant,
      FlowJumpToAddressPlusVar { constant } => {
        self.program_counter = (self.v_registers[0] as u16) + constant
      }
      RandomByConstant {
        x_register,
        constant,
      } => unimplemented!("TODO(joey): implement"),
      DisplayDraw {
        x_register,
        y_register,
        constant,
      } => unimplemented!("TODO(joey): implement"),
      InputKeyIsPressed { x_register } => unimplemented!("TODO(joey): implement"),
      InputKeyIsNotPressed { x_register } => unimplemented!("TODO(joey): implement"),
      TimerGetDelay { x_register } => unimplemented!("TODO(joey): implement"),
      InputKeyAwaitPress { x_register } => unimplemented!("TODO(joey): implement"),
      TimerSetDelay { x_register } => self.delay_timer.set_counter(self.v_registers[x_register]),
      TimerSetSound { x_register } => self.sound_timer.set_counter(self.v_registers[x_register]),
      MemoryAddVerToAddress { x_register } => {
        self.i_register += self.v_registers[x_register] as u16
      }
      MemorySetToVarSpriteLocation { x_register } => unimplemented!("TODO(joey): implement"),
      LoadBinaryCodedDecimal { x_register } => unimplemented!("TODO(joey): implement"),
      MemoryDump { x_register } => {
        // FIXME(joey): Check bounds on iteration, this should be inclusive of `x_register`.
        for register in 0..x_register {
          self.memory[self.i_register as usize + register] = self.v_registers[register]
        }
      }
      MemoryLoad { x_register } => {
        // FIXME(joey): Check bounds on iteration, this should be inclusive of `x_register`.
        for register in 0..x_register {
          self.v_registers[register] = self.memory[self.i_register as usize + register]
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  use crate::interface::serialization;

  #[test]
  fn serialization_roundtrip_works() -> std::io::Result<()> {
    let buf = &mut Vec::new();
    let cpu = &Cpu::new();
    cpu.save(buf)?;
    let loaded_cpu = &serialization::read_value::<Cpu>(&mut buf.as_slice())?;
    assert_eq!(cpu, loaded_cpu);
    Ok(())
  }
}

// use display::Display;
// use keypad::Keypad;
// use opcode::Opcode;
// use util;

// pub struct Cpu {
//   pub memory: [u8; 4096],
//   pub v: [u8; 16],
//   pub i: usize,
//   pub pc: usize,
//   pub gfx: [u8; 64 * 32],
//   pub delay_timer: u8,
//   pub sound_timer: u8,
//   pub stack: [u16; 16],
//   pub sp: usize,
//   pub keypad: Keypad,
//   pub display: Display,
// }

// fn not_implemented(op: usize, pc: usize) {
//   println!("Not implemented:: op: {:x}, pc: {:x}", op, pc);
// }

// impl Cpu {
//   pub fn new(&context: &sdl2::Sdl) -> Cpu {
//     let mut cpu: Cpu = Cpu {
//       v: [0; 16],
//       gfx: [0; 64 * 32],
//       delay_timer: 0,
//       sound_timer: 0,
//       stack: [0; 16],

//       pc: 0x200, // Program counter starts at 0x200
//       i: 0,      // Reset index register
//       sp: 0,     // Reset stack pointer
//       memory: [0; 4096],

//       display: Display::new(context),
//       keypad: Keypad::new(),
//     };
//     // load in Util::fontset
//     for i in 0..80 {
//       cpu.memory[i] = util::FONTSET[i];
//     }
//     return cpu;
//   }

//   pub fn emulate_cycle(&mut self) {
//     // fetch opcode
//     let opcode_bits: u16 =
//       (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
//     let opcode: Opcode = Opcode::new(opcode_bits);

//     util::debug_cycle(self, &opcode);

//     self.pc += 2;

//     self.execute(&opcode);

//     // ??
//     // self.pc += 2;

//     // update timers
//     if self.delay_timer > 0 {
//       self.delay_timer -= 1;
//     }
//     if self.sound_timer > 0 {
//       if self.sound_timer == 0 {
//         println!("BUZZ");
//       }
//       self.sound_timer -= 1;
//     }
//   }

//   pub fn execute(&mut self, opcode: &Opcode) {
//     match opcode.code & 0xf000 {
//       0x0000 => self.execute_clear_return(opcode), // 0nnn - SYS  nnn
//       0x1000 => op_1xxx(self, opcode),
//       0x2000 => op_2xxx(self, opcode),
//       0x3000 => op_3xxx(self, opcode),
//       0x4000 => op_4xxx(self, opcode),
//       0x5000 => op_5xxx(self, opcode),
//       0x6000 => op_6xxx(self, opcode),
//       0x7000 => op_7xxx(self, opcode),
//       0x8000 => op_8xxx(self, opcode),
//       0x9000 => op_9xxx(self, opcode),
//       0xA000 => op_Axxx(self, opcode),
//       0xB000 => op_Bxxx(self, opcode),
//       0xC000 => op_Cxxx(self, opcode),
//       0xD000 => op_Dxxx(self, opcode),
//       0xE000 => op_Exxx(self, opcode),
//       0xF000 => op_Fxxx(self, opcode),
//       _ => not_implemented(opcode.code as usize, self.pc),
//     }
//   }
//   pub fn execute_clear_return(&mut self, opcode: &Opcode) {
//     match opcode.code {
//       0x00E0 => {
//         // CLS
//         // clear screen
//         self.display.clear();
//       }
//       0x00EE => self.return_from_subroutine(),
//       _ => not_implemented(opcode.code as usize, self.pc),
//     }
//   }

//   pub fn return_from_subroutine(&mut self) {
//     // 00EE - RTS
//     // Return from subroutine. Pop the current value in the stack pointer
//     // off of the stack, and set the program counter to the value popped.
//     // self.registers['sp'] -= 1
//     // self.registers['pc'] = self.memory[self.registers['sp']] << 8
//     // self.registers['sp'] -= 1
//     // self.registers['pc'] += self.memory[self.registers['sp']]
//     self.sp -= 1;
//   }

//   //     0x0: self.clear_return,                  # 0nnn - SYS  nnn
//   //          0x1: self.jump_to_address,               # 1nnn - JUMP nnn
//   //          0x2: self.jump_to_subroutine,            # 2nnn - CALL nnn
//   //          0x3: self.skip_if_reg_equal_val,         # 3snn - SKE  Vs, nn
//   //          0x4: self.skip_if_reg_not_equal_val,     # 4snn - SKNE Vs, nn
//   //          0x5: self.skip_if_reg_equal_reg,         # 5st0 - SKE  Vs, Vt
//   //          0x6: self.move_value_to_reg,             # 6snn - LOAD Vs, nn
//   //          0x7: self.add_value_to_reg,              # 7snn - ADD  Vs, nn
//   //          0x8: self.execute_logical_instruction,   # see subfunctions below
//   //          0x9: self.skip_if_reg_not_equal_reg,     # 9st0 - SKNE Vs, Vt
//   //          0xA: self.load_index_reg_with_value,     # Annn - LOAD I, nnn
//   //          0xB: self.jump_to_index_plus_value,      # Bnnn - JUMP [I] + nnn
//   //          0xC: self.generate_random_number,        # Ctnn - RAND Vt, nn
//   //          0xD: self.draw_sprite,                   # Dstn - DRAW Vs, Vy, n
//   //          0xE: self.keyboard_routines,             # see subfunctions below
//   //          0xF: self.misc_routines,                 # see subfunctions below
// }

// fn op_1xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // JP addr
//   // jumps to addr
//   cpu.pc = opcode.address;
// }

// fn op_2xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // CALL addr
//   // calls subroutine at addr
//   cpu.stack[cpu.sp] = opcode.address as u16;
//   cpu.sp += 1;
// }

// fn op_3xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // SE Vx, byte
//   // skips next instruction if VX eq NN
//   if cpu.v[opcode.x] == opcode.const8bit {
//     cpu.pc += 2;
//   }
// }

// fn op_4xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // SNE Vx, byte
//   // skips next instruction if VX neq MM
//   if cpu.v[opcode.x] != opcode.const8bit {
//     cpu.pc += 2;
//   }
// }

// fn op_5xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // SE Vx, Vy
//   // skips next instruction if VX eq VY
//   if cpu.v[opcode.x] != opcode.const8bit {
//     cpu.pc += 2;
//   }
// }

// fn op_6xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // LD Vx, byte
//   // sets VX to NN
//   cpu.v[opcode.x] = opcode.const8bit;
// }

// fn op_7xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // ADD Vx, byte
//   // incr VX by NN
//   cpu.v[opcode.x] += opcode.const8bit;
// }

// fn op_8xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   match opcode.code & 0x000F {
//     0x0000 => {
//       // LD Vx, Vy
//       // set VX to VY
//       cpu.v[opcode.x] += cpu.v[opcode.y];
//     }
//     0x0001 => {
//       // OR Vx, Vy
//       // sets VX to VX or VY
//       cpu.v[opcode.x] |= cpu.v[opcode.y];
//     }
//     0x0002 => {
//       // AND Vx, Vy
//       // sets VX to VX and VY
//       cpu.v[opcode.x] &= cpu.v[opcode.y];
//     }
//     0x0003 => {
//       // XOR Vx, Vy
//       // sets VX to VX and VY
//       cpu.v[opcode.x] ^= cpu.v[opcode.y];
//     }
//     0x0004 => {
//       // ADD Vx, Vy
//       // incr VX by VY, if VF = 1 if carry else 0
//       if cpu.v[opcode.y] < (0xFF - cpu.v[opcode.x]) {
//         cpu.v[0xF] = 1;
//       } else {
//         cpu.v[0xF] = 0;
//       };

//       cpu.v[opcode.x] = (cpu.v[opcode.x] as u16 + cpu.v[opcode.y] as u16) as u8;
//     }
//     0x0005 => {
//       // SUB Vx, Vy
//       // sub VX by VY, if VF = 0 if borrow else 1
//       if cpu.v[opcode.y] > cpu.v[opcode.x] {
//         cpu.v[0xF] = 1;
//       } else {
//         cpu.v[0xF] = 0;
//       }

//       cpu.v[opcode.x] -= cpu.v[opcode.y];
//     }
//     0x0006 => {
//       // SHR Vx {, Vy}
//       // right ship VX by 1. Set VF to the least significant bit before shifting
//       cpu.v[0xF] = cpu.v[opcode.x] & 0x0001;
//       cpu.v[opcode.x] >> 1;
//     }
//     0x0007 => {
//       // SUBN Vx, Vy
//       // sub VX by VY, if VF = 0 if borrow else 1
//       if cpu.v[opcode.x] > cpu.v[opcode.y] {
//         cpu.v[0xF] = 1;
//       } else {
//         cpu.v[0xF] = 0;
//       }

//       cpu.v[opcode.x] = cpu.v[opcode.y] - cpu.v[opcode.x];
//     }
//     0x000E => {
//       // SHL Vx {, Vy}
//       // Set Vx = Vx SHL 1.
//       // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
//       // Then Vx is multiplied by 2.
//       // TODO
//     }
//     _ => not_implemented(opcode.code as usize, cpu.pc),
//   }
// }

// fn op_9xxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // SNE Vx, Vy
//   // skips the next instruction if VX neq Vy
//   if cpu.v[opcode.x] != cpu.v[opcode.y] {
//     cpu.pc += 2;
//   }
// }

// fn op_Axxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // LD I, addr
//   // sets I to addr
//   cpu.i = opcode.address;
// }

// fn op_Bxxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // JP V0, addr
//   // jumps to address + V0
//   cpu.pc = opcode.address + cpu.v[0x0] as usize;
// }

// fn op_Cxxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // RND Vx, byte
//   // sets VX to a random number, masked by NN
//   let rand = 0;
//   cpu.v[opcode.x] = rand & opcode.const8bit;
// }

// fn op_Dxxx(cpu: &mut Cpu, opcode: &Opcode) {
//   // DRW Vx, Vy, nibble
//   // Sprites stored in memory at location in index register (I), maximum 8bits wide.
//   // Wraps around the screen. If when drawn, clears a pixel,
//   // register VF is set to 1 otherwise it is zero.
//   // All drawing is XOR drawing (i.e. it toggles the screen pixels)

//   let from: usize = cpu.i;
//   let to: usize = from + opcode.const4bit as usize;
//   let x: u8 = cpu.v[opcode.x];
//   let y: u8 = cpu.v[opcode.y];
//   cpu.v[15] = cpu
//     .display
//     .draw(x as usize, y as usize, &cpu.memory[from..to]);
//   cpu.pc += 2;

//   cpu.v[0xF] = 0;

//   let height = opcode.code & 0x000F;
//   let regX = cpu.v[opcode.x] as usize;
//   let regY = cpu.v[opcode.y] as usize;
//   let mut spr;

//   for y in 0..height {
//     spr = cpu.memory[cpu.i + y as usize];
//     for x in 0..8 {
//       if (spr & 0x80) > 0 {
//         cpu.display.gfx[regX + x][regY + y as usize] =
//           cpu.display.gfx[regX + x][regY + y as usize] ^ 1;
//         if cpu.display.gfx[regX + x as usize][regY + y as usize] == 1 {
//           cpu.v[0xF] = 1;
//         }
//       }
//       spr <<= 1;
//     }
//   }
//   cpu.display.draw_flag = true;
// }

// fn op_Exxx(cpu: &mut Cpu, opcode: &Opcode) {
//   match opcode.code & 0x00FF {
//     0x009E => {
//       // SKP Vx
//       // Skips the next instruction if the key stored in VX is pressed.
//       if cpu.keypad.pressed(cpu.v[opcode.x] as usize) {
//         cpu.pc += 2;
//       }
//     }
//     0x00A1 => {
//       // SKNP Vx
//       // Skips the next instruction if the key stored in VX isn't pressed.
//       if !cpu.keypad.pressed(cpu.v[opcode.x] as usize) {
//         cpu.pc += 2;
//       }
//     }
//     _ => not_implemented(opcode.code as usize, cpu.pc),
//   }
// }

// fn op_Fxxx(cpu: &mut Cpu, opcode: &Opcode) {
//   match opcode.code & 0x00FF {
//     0x0007 => {
//       // LD Vx, DT
//       // Sets VX to the value of the delay timer.
//       cpu.v[opcode.x] = cpu.delay_timer;
//     }
//     0x000A => {
//       // LD Vx, K
//       // A key press is awaited, and then stored in VX.
//       for i in 0u8..16 {
//         if cpu.keypad.pressed(i as usize) {
//           cpu.v[opcode.x] = i;
//           break;
//         }
//       }
//       cpu.pc -= 2;
//     }
//     0x0015 => {
//       // LD DT, Vx
//       // Sets the delay timer to VX.
//       cpu.delay_timer = cpu.v[opcode.x];
//     }
//     0x0018 => {
//       // LD ST, Vx
//       // Sets the sound timer to VX.
//       cpu.sound_timer = cpu.v[opcode.x];
//     }
//     0x001E => {
//       // ADD I, Vx
//       // adds VX to I

//       // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
//       // This is undocumented feature of the CHIP-8 and used by Spacefight 2091! game.
//       if cpu.i as u16 + (cpu.v[opcode.x] as u16) > 0xFFF {
//         cpu.v[0xF] = 1;
//       } else {
//         cpu.v[0xF] = 0;
//       }

//       cpu.i += cpu.v[opcode.x] as usize;
//     }
//     0x0029 => {
//       // LD F, Vx
//       // Sets I to the location of the sprite for the character in VX.
//       // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
//     }
//     0x0033 => {
//       // LD B, Vx
//       // Stores the Binary-coded decimal representation of VX,
//       // with the most significant of three digits at the address in I,
//       // the middle digit at I plus 1, and the least significant digit at I plus 2.
//       // (In other words, take the decimal representation of VX, place the hundreds
//       // digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
//     }
//     0x0055 => {
//       // LD [I], Vx
//       // Stores V0 to VX in memory starting at address I.
//       for i in 0..cpu.v[opcode.x] {
//         cpu.memory[cpu.i + i as usize] = cpu.v[i as usize];
//       }
//     }
//     0x0065 => {
//       // LD Vx, [I]
//       // Fills V0 to VX with values from memory starting at address I.
//       for i in 0..cpu.v[opcode.x] {
//         cpu.v[i as usize] = cpu.memory[cpu.i + i as usize];
//       }
//     }
//     _ => not_implemented(opcode.code as usize, cpu.pc),
//   }
// }
