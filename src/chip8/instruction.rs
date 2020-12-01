//! instruction provides parsing for the Chip8 machine language as well
//! as struct representations, used for emulation.
//!
//! References include:
//! * Wikipedia Chip8 Reference: https://en.wikipedia.org/wiki/CHIP-8
//! * Cowgod's Technical Reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0nnn

use thiserror::Error;

/// TODO(joey): Document InstructionError.
#[derive(Error, Debug)]
pub enum InstructionError {
  /// TODO(joey): Document UnsupportedInstructionError.
  #[error("Unsupported instruction error")]
  UnsupportedInstructionError { instruction: u16 },
}

/// Instruction is the enum (and struct) for all Chip8 opcodes along
/// with the operands corresponding to the opcode.
///
/// Each opcodes is documented in hexadecimal and with the following
/// symbols:
///
/// * NNN: address
/// * NN: 8-bit constant
/// * N: 4-bit constant
/// * X and Y: 4-bit register identifier
/// * PC : Program Counter
/// * I : 16bit register (For memory address) (Similar to void pointer)
/// * VN: One of the 16 available variables. N may be 0 to F
///   (hexadecimal)
//
// TODO(joey): It may be helpful to specialize the byte sizes for other
// data sizes, such as 12-bit or 4-bit data.
//
// TODO(joey): It would be cute (but probably far too much work) to
// develop a doc macro that expands test cases based on the macros
// opcode, using generative testing. For example, with the notation 3XNN
// we know:
// * The opcode is defined by 0x3000 using the mask 0xF000.
// * The opcode should have an x_register field, equal to X.
// * The opcode should have a constant field, equal to NN.
//
// This may be worth it if I expand a similar instruction definition to
// more emulators!
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
  /// Opcode 0NNN
  ///
  /// From Wikipedia: Calls machine code routine (RCA 1802 for COSMAC
  /// VIP) at address NNN. Not necessary for most ROMs.
  ///
  ///
  /// From Cowgod's Technical Reference: This instruction is only used
  /// on the old computers on which Chip-8 was originally implemented.
  /// It is ignored by modern interpreters.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x0111);
  /// assert!(matches!(instr, Instruction::Call {..} ), "Expected to parse Call, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::Call { address: 0x0111 });
  /// ```
  Call { address: usize },
  /// Opcode 00E0
  ///
  /// Clears the screen.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x00E0);
  /// assert!(matches!(instr, Instruction::DisplayClear(..)), "Expected to parse DisplayClear, instead parsed opcode: {:?}", instr);
  /// ```
  DisplayClear(),
  /// Opcode 00EE
  ///
  /// Returns from the subroutine.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x00EE);
  /// assert!(matches!(instr, Instruction::FlowSubroutineReturn(..)), "Expected to parse FlowSubroutineReturn, instead parsed opcode: {:?}", instr);
  /// ```
  FlowSubroutineReturn(),
  /// Opcode 1NNN
  ///
  /// Jumps to the address NNN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x1234);
  /// assert!(matches!(instr, Instruction::FlowJumpToAddress {..}), "Expected to parse FlowJumpToAddress, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::FlowJumpToAddress { address: 0x0234 });
  /// ```
  FlowJumpToAddress { address: usize },
  /// Opcode 2NNN
  ///
  /// Calls the subroutine at NNN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x2345);
  /// assert!(matches!(instr, Instruction::FlowSubroutineCall {..}), "Expected to parse FlowSubroutineCall, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::FlowSubroutineCall { address: 0x0345 });
  /// ```
  FlowSubroutineCall { address: usize },
  /// Opcode 3XNN
  ///
  /// Skips the next instruction if the value in Vx is equal to NN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x3E45);
  /// assert!(matches!(instr, Instruction::CondSkipIfEqualConst {..}), "Expected to parse CondSkipIfEqualConst, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::CondSkipIfEqualConst { x_register: 0xE, constant: 0x45 });
  /// ```
  CondSkipIfEqualConst { x_register: usize, constant: u8 },
  /// Opcode 4XNN
  ///
  /// Skips the next instruction if the value in Vx is *NOT* equal to NN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x4E45);
  /// assert!(matches!(instr, Instruction::CondSkipIfNotEqualConst {..}), "Expected to parse CondSkipIfNotEqualConst, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::CondSkipIfNotEqualConst { x_register: 0xE, constant: 0x45 });
  /// ```
  CondSkipIfNotEqualConst { x_register: usize, constant: u8 },
  /// Opcode 5XY0
  ///
  /// Skips the next instruction if the value in Vx is equal to Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x5E40);
  /// assert!(matches!(instr, Instruction::CondSkipIfEqualVar {..}), "Expected to parse CondSkipIfEqualVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::CondSkipIfEqualVar { x_register: 0xE, y_register: 0x4 });
  /// ```
  CondSkipIfEqualVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 6XNN
  ///
  /// Sets Vx to the value of NN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x6E42);
  /// assert!(matches!(instr, Instruction::ConstSetVar {..}), "Expected to parse ConstSetVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::ConstSetVar { x_register: 0xE, constant: 0x42 });
  /// ```
  ConstSetVar { x_register: usize, constant: u8 },
  /// Opcode 7XNN
  ///
  /// Increments Vx by the value NN.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x7E42);
  /// assert!(matches!(instr, Instruction::ConstIncrementVar {..}), "Expected to parse ConstIncrementVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::ConstIncrementVar { x_register: 0xE, constant: 0x42 });
  /// ```
  ConstIncrementVar { x_register: usize, constant: u8 },
  /// Opcode 8XY0
  ///
  /// Sets Vx to the value of Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A0);
  /// assert!(matches!(instr, Instruction::AssignVar {..}), "Expected to parse AssignVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::AssignVar { x_register: 0x9, y_register: 0xA });
  /// ```
  AssignVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY1
  ///
  /// Sets Vx to the bitwise OR of Vx and Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A1);
  /// assert!(matches!(instr, Instruction::BitwiseOrVar {..}), "Expected to parse BitwiseOrVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::BitwiseOrVar { x_register: 0x9, y_register: 0xA });
  /// ```
  BitwiseOrVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY2
  ///
  /// Sets Vx to the bitwise AND of Vx and Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A2);
  /// assert!(matches!(instr, Instruction::BitwiseAndVar {..}), "Expected to parse BitwiseAndVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::BitwiseAndVar { x_register: 0x9, y_register: 0xA });
  /// ```
  BitwiseAndVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY3
  ///
  /// Sets Vx to the bitwise XOR of Vx and Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A3);
  /// assert!(matches!(instr, Instruction::BitwiseXorVar {..}), "Expected to parse BitwiseXorVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::BitwiseXorVar { x_register: 0x9, y_register: 0xA });
  /// ```
  BitwiseXorVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY4
  ///
  /// Adds VY to VX. VF is set to 1 when there is a carry, and to 0 when
  /// there is not.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A4);
  /// assert!(matches!(instr, Instruction::MathAddVar {..}), "Expected to parse MathAddVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MathAddVar { x_register: 0x9, y_register: 0xA });
  /// ```
  MathAddVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY5
  ///
  /// Sets Vx to the Vx minus Vy.
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A5);
  /// assert!(matches!(instr, Instruction::MathSubVar {..}), "Expected to parse MathSubVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MathSubVar { x_register: 0x9, y_register: 0xA });
  /// ```
  MathSubVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY6
  ///
  /// Stores the least significant bit of VX in VF and then shifts VX to
  /// the right by 1.
  ///
  /// NB: CHIP-8's opcodes 8XY6 and 8XYE (the bit shift instructions),
  /// which were in fact undocumented opcodes in the original
  /// interpreter, shifted the value in the register VY and stored the
  /// result in VX. The CHIP-48 and SCHIP implementations instead
  /// ignored VY, and simply shifted VX.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A6);
  /// assert!(matches!(instr, Instruction::BitShiftRightVar {..}), "Expected to parse BitShiftRightVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::BitShiftRightVar { x_register: 0x9, y_register: 0xA });
  /// ```
  BitShiftRightVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XY7
  ///
  /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and
  /// 1 when there is not.
  ///
  /// NB: CHIP-8's opcodes 8XY6 and 8XYE (the bit shift instructions),
  /// which were in fact undocumented opcodes in the original
  /// interpreter, shifted the value in the register VY and stored the
  /// result in VX. The CHIP-48 and SCHIP implementations instead
  /// ignored VY, and simply shifted VX.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89A7);
  /// assert!(matches!(instr, Instruction::MathReverseSubtractVar {..}), "Expected to parse MathReverseSubtractVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MathReverseSubtractVar { x_register: 0x9, y_register: 0xA });
  /// ```
  MathReverseSubtractVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 8XYE
  ///
  /// Stores the least significant bit of Vx in Vf and then shifts Vx to
  /// the left by 1.
  ///
  /// NB: See BitShiftRightVar comment for system compatibility.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x89AE);
  /// assert!(matches!(instr, Instruction::BitShiftLeftVar {..}), "Expected to parse BitShiftLeftVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::BitShiftLeftVar { x_register: 0x9, y_register: 0xA });
  /// ```
  BitShiftLeftVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode 9XY0
  ///
  /// Skips the next instruction if Vx doesn't equal Vy.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0x98A0);
  /// assert!(matches!(instr, Instruction::CondSkipIfNotEqualVar {..}), "Expected to parse CondSkipIfNotEqualVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::CondSkipIfNotEqualVar { x_register: 0x8, y_register: 0xA });
  /// ```
  CondSkipIfNotEqualVar {
    x_register: usize,
    y_register: usize,
  },
  /// Opcode ANNN
  ///
  /// Sets I (address register) to the address NNN.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xA123);
  /// assert!(matches!(instr, Instruction::MemorySetAddress {..}), "Expected to parse MemorySetAddress, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MemorySetAddress { constant: 0x123 });
  /// ```
  MemorySetAddress { constant: u16 },
  /// Opcode BNNN
  ///
  /// Jumps to the address constant (NNN) plus V0
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xB123);
  /// assert!(matches!(instr, Instruction::FlowJumpToAddressPlusVar {..}), "Expected to parse FlowJumpToAddressPlusVar, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::FlowJumpToAddressPlusVar { constant: 0x123 });
  /// ```
  FlowJumpToAddressPlusVar { constant: u16 },
  /// Opcode CXNN
  ///
  /// Sets Vx to the result of a bitwise AND operation on a random
  /// number (Typically: 0 to 255) and NN.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xC123);
  /// assert!(matches!(instr, Instruction::RandomByConstant {..}), "Expected to parse RandomByConstant, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::RandomByConstant { x_register: 0x1, constant: 0x23 });
  /// ```
  RandomByConstant { x_register: usize, constant: u8 },
  /// Opcode DXYN
  ///
  /// Draws a sprite at coordinate (Vx, Vy) that has a width of 8 pixels
  /// and a height of N+1 pixels. Each row of 8 pixels is read as
  /// bit-coded starting from memory location of the address register
  /// (I); The I register value does not change after the execution of
  /// this instruction. As described above, Vf is set to 1 if any screen
  /// pixels are flipped from SET to UNSET when the sprite is drawn, and
  /// to 0 if that does not happen.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xD123);
  /// assert!(matches!(instr, Instruction::DisplayDraw {..}), "Expected to parse DisplayDraw, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::DisplayDraw { x_register: 0x1, y_register: 0x2, constant: 0x3 });
  /// ```
  DisplayDraw {
    x_register: usize,
    y_register: usize,
    constant: u8,
  },
  /// Opcode EX9E
  ///
  /// Skips the next instruction if the key (input) stored in Vx is
  /// pressed.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xE19E);
  /// assert!(matches!(instr, Instruction::InputKeyIsPressed {..}), "Expected to parse InputKeyIsPressed, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::InputKeyIsPressed { x_register: 0x1 });
  /// ```
  InputKeyIsPressed { x_register: usize },
  /// Opcode EXA1
  ///
  /// Skips the next instruction if the key (input) stored in Vx is
  /// pressed.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xE1A1);
  /// assert!(matches!(instr, Instruction::InputKeyIsNotPressed {..}), "Expected to parse InputKeyIsNotPressed, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::InputKeyIsNotPressed { x_register: 0x1 });
  /// ```
  InputKeyIsNotPressed { x_register: usize },
  /// Opcode FX07
  ///
  /// Sets Vx to the value of the delay timer.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF107);
  /// assert!(matches!(instr, Instruction::TimerGetDelay {..}), "Expected to parse TimerGetDelay, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::TimerGetDelay { x_register: 0x1 });
  /// ```
  TimerGetDelay { x_register: usize },
  /// Opcode FX0A
  ///
  /// A key press is awaited, and then stored in Vx. This instruction is
  /// a blocking operation. All instructions are halted until the next
  /// key event.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF10A);
  /// assert!(matches!(instr, Instruction::InputKeyAwaitPress {..}), "Expected to parse InputKeyAwaitPress, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::InputKeyAwaitPress { x_register: 0x1 });
  /// ```
  InputKeyAwaitPress { x_register: usize },
  /// Opcode FX15
  ///
  /// Sets the delay timer to Vx.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF215);
  /// assert!(matches!(instr, Instruction::TimerSetDelay {..}), "Expected to parse TimerSetDelay, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::TimerSetDelay { x_register: 0x2 });
  /// ```
  TimerSetDelay { x_register: usize },
  /// Opcode FX18
  ///
  /// Sets the sound timer to Vx.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF218);
  /// assert!(matches!(instr, Instruction::TimerSetSound {..}), "Expected to parse TimerSetSound, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::TimerSetSound { x_register: 0x2 });
  /// ```
  TimerSetSound { x_register: usize },
  /// Opcode FX1E
  ///
  /// Adds Vx to the address register (I).
  ///
  /// NB: Vf is not affected. Most CHIP-8 interpreters' FX1E
  /// instructions do not affect Vf, with one exception: The CHIP-8
  /// interpreter for the Commodore Amiga sets Vf to 1 when there is a
  /// range overflow (I+Vx>0xFFF), and to 0 when there isn't. The only
  /// known game that depends on this behavior is Spacefight 2091! while
  /// at least one game, Animal Race, depends on Vf not being affected,
  /// according to:
  /// https://github.com/Chromatophore/HP48-Superchip/issues/2.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF21E);
  /// assert!(matches!(instr, Instruction::MemoryAddVerToAddress {..}), "Expected to parse MemoryAddVerToAddress, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MemoryAddVerToAddress { x_register: 0x2 });
  /// ```
  MemoryAddVerToAddress { x_register: usize },
  /// Opcode FX29
  ///
  /// Sets address register (I) to the location of the sprite for the
  /// character in Vx. Characters 0-F (in hexadecimal) are represented
  /// by a 4x5 font.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF229);
  /// assert!(matches!(instr, Instruction::MemorySetToVarSpriteLocation {..}), "Expected to parse MemorySetToVarSpriteLocation, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MemorySetToVarSpriteLocation { x_register: 0x2 });
  /// ```
  MemorySetToVarSpriteLocation { x_register: usize },
  /// Opcode FX33
  ///
  /// Stores the binary-coded decimal representation of Vx, with the
  /// most significant of three digits at the address in I, the middle
  /// digit at I plus 1, and the least significant digit at I plus 2.
  /// (In other words, take the decimal representation of Vx, place the
  /// hundreds digit in memory at location in I, the tens digit at
  /// location I+1, and the ones digit at location I+2.)
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF233);
  /// assert!(matches!(instr, Instruction::LoadBinaryCodedDecimal {..}), "Expected to parse LoadBinaryCodedDecimal, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::LoadBinaryCodedDecimal { x_register: 0x2 });
  /// ```
  LoadBinaryCodedDecimal { x_register: usize },
  /// Opcode FX55
  ///
  /// Stores V0 to Vx (including Vx) in memory starting at address I.
  /// The offset from I is increased by 1 for each value written, but I
  /// itself is left unmodified.
  ///
  /// NB: In the original CHIP-8 implementation, and also in CHIP-48, I
  /// is left incremented after this instruction had been executed. In
  /// SCHIP, I is left unmodified.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF255);
  /// assert!(matches!(instr, Instruction::MemoryDump {..}), "Expected to parse MemoryDump, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MemoryDump { x_register: 0x2 });
  /// ```
  MemoryDump { x_register: usize },
  /// Opcode FX65
  ///
  /// Fills V0 to Vx (including Vx) with values from memory starting at
  /// address I. The offset from I is increased by 1 for each value
  /// written, but I itself is left unmodified.
  ///
  /// NB: See note for MemoryDump for system compability.
  ///
  /// ```
  /// # use rustyemulator::chip8::instruction::{parse_instruction, Instruction};
  /// let instr = parse_instruction(0xF265);
  /// assert!(matches!(instr, Instruction::MemoryLoad {..}), "Expected to parse MemoryLoad, instead parsed opcode: {:?}", instr);
  /// assert_eq!(instr, Instruction::MemoryLoad { x_register: 0x2 });
  /// ```
  MemoryLoad { x_register: usize },
}

/// parse_instruction will parse a Chip8 instruction (2 bytes) into the
/// Instruction enum.
pub fn parse_instruction(instr: u16) -> Result<Instruction, InstructionError> {
  match instr & 0xF000 {
    0x0000 => {
      // Because Call has overlapping instructions with DisplayClear and
      // FlowSubroutineReturn, the more specific opcodes (DisplayClear,
      // FlowSubroutineReturn) are parsed first.
      match instr {
        0x00E0 => Ok(Instruction::DisplayClear()),
        0x00EE => Ok(Instruction::FlowSubroutineReturn()),
        _ => Ok(Instruction::Call {
          address: bitmask_0NNN(instr) as usize,
        }),
      }
    }
    0x1000 => Ok(Instruction::FlowJumpToAddress {
      address: bitmask_0NNN(instr) as usize,
    }),
    0x2000 => Ok(Instruction::FlowSubroutineCall {
      address: bitmask_0NNN(instr) as usize,
    }),
    0x3000 => Ok(Instruction::CondSkipIfEqualConst {
      x_register: bitmask_0X00(instr) as usize,
      constant: bitmask_00NN(instr),
    }),
    0x4000 => Ok(Instruction::CondSkipIfNotEqualConst {
      x_register: bitmask_0X00(instr) as usize,
      constant: bitmask_00NN(instr),
    }),
    0x5000 => match instr & 0x000F {
      0x0000 => Ok(Instruction::CondSkipIfEqualVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
    },
    0x6000 => Ok(Instruction::ConstSetVar {
      x_register: bitmask_0X00(instr) as usize,
      constant: bitmask_00NN(instr),
    }),
    0x7000 => Ok(Instruction::ConstIncrementVar {
      x_register: bitmask_0X00(instr) as usize,
      constant: bitmask_00NN(instr),
    }),
    0x8000 => match instr & 0x000F {
      0x0000 => Ok(Instruction::AssignVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0001 => Ok(Instruction::BitwiseOrVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0002 => Ok(Instruction::BitwiseAndVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0003 => Ok(Instruction::BitwiseXorVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0004 => Ok(Instruction::MathAddVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0005 => Ok(Instruction::MathSubVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0006 => Ok(Instruction::BitShiftRightVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x0007 => Ok(Instruction::MathReverseSubtractVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      0x000E => Ok(Instruction::BitShiftLeftVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
    },
    0x9000 => match instr & 0x000F {
      0x0000 => Ok(Instruction::CondSkipIfNotEqualVar {
        x_register: bitmask_0X00(instr) as usize,
        y_register: bitmask_00Y0(instr) as usize,
      }),
      _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
    },
    0xA000 => Ok(Instruction::MemorySetAddress {
      constant: bitmask_0NNN(instr),
    }),
    0xB000 => Ok(Instruction::FlowJumpToAddressPlusVar {
      constant: bitmask_0NNN(instr),
    }),
    0xC000 => Ok(Instruction::RandomByConstant {
      x_register: bitmask_0X00(instr) as usize,
      constant: bitmask_00NN(instr),
    }),
    0xD000 => Ok(Instruction::DisplayDraw {
      x_register: bitmask_0X00(instr) as usize,
      y_register: bitmask_00Y0(instr) as usize,
      constant: bitmask_000N(instr),
    }),
    0xE000 => match instr & 0x00FF {
      0x009E => Ok(Instruction::InputKeyIsPressed {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x00A1 => Ok(Instruction::InputKeyIsNotPressed {
        x_register: bitmask_0X00(instr) as usize,
      }),
      _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
    },
    0xF000 => match instr & 0x00FF {
      0x0007 => Ok(Instruction::TimerGetDelay {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x000A => Ok(Instruction::InputKeyAwaitPress {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0015 => Ok(Instruction::TimerSetDelay {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0018 => Ok(Instruction::TimerSetSound {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x001E => Ok(Instruction::MemoryAddVerToAddress {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0029 => Ok(Instruction::MemorySetToVarSpriteLocation {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0033 => Ok(Instruction::LoadBinaryCodedDecimal {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0055 => Ok(Instruction::MemoryDump {
        x_register: bitmask_0X00(instr) as usize,
      }),
      0x0065 => Ok(Instruction::MemoryLoad {
        x_register: bitmask_0X00(instr) as usize,
      }),
      _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
    },
    _ => Err(InstructionError::UnsupportedInstructionError { instruction: instr }),
  }
}

#[allow(non_snake_case)]
fn bitmask_0NNN(instr: u16) -> u16 {
  instr & 0x0FFF
}

#[allow(non_snake_case)]
fn bitmask_00NN(instr: u16) -> u8 {
  (instr & 0x00FF) as u8
}

#[allow(non_snake_case)]
fn bitmask_000N(instr: u16) -> u8 {
  (instr & 0x000F) as u8
}

#[allow(non_snake_case)]
fn bitmask_0X00(instr: u16) -> u8 {
  ((instr & 0x0F00) >> 8) as u8
}

#[allow(non_snake_case)]
fn bitmask_00Y0(instr: u16) -> u8 {
  ((instr & 0x00F0) >> 4) as u8
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  #[allow(non_snake_case)]
  fn bitmask_0NNN_works() {
    assert_eq!(0x0, bitmask_0NNN(0x0000));
    assert_eq!(0x234, bitmask_0NNN(0x1234));
  }

  #[test]
  #[allow(non_snake_case)]
  fn bitmask_00NN_works() {
    assert_eq!(0x0, bitmask_00NN(0x0000));
    assert_eq!(0x34, bitmask_00NN(0x1234));
  }

  #[test]
  #[allow(non_snake_case)]
  fn bitmask_000N_works() {
    assert_eq!(0x0, bitmask_000N(0x0000));
    assert_eq!(0x4, bitmask_000N(0x1234));
  }

  #[test]
  #[allow(non_snake_case)]
  fn bitmask_0X00_works() {
    assert_eq!(0x0, bitmask_0X00(0x0000));
    assert_eq!(0x2, bitmask_0X00(0x1234));
  }

  #[test]
  #[allow(non_snake_case)]
  fn bitmask_00Y0_works() {
    assert_eq!(0x0, bitmask_00Y0(0x0000));
    assert_eq!(0x3, bitmask_00Y0(0x1234));
  }
}
