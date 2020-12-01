use crate::chip8::cpu::Cpu;
use crate::interface::emulator::Clocked;
use crate::interface::serialization::Savable;

// Chip8 is the root emulation tree for the Chip8 system.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Chip8 {
  cpu: Cpu,
}

impl Savable for Chip8 {
  fn save(&self, fh: &mut dyn std::io::Write) -> std::io::Result<()> {
    self.cpu.save(fh)?;
    Ok(())
  }
  fn load(&mut self, fh: &mut dyn std::io::Read) -> std::io::Result<()> {
    self.cpu.load(fh)?;
    Ok(())
  }
}

impl Clocked for Chip8 {
  fn clock(&mut self) {
    unimplemented!("TODO(joey): implement");
  }

  fn clock_rate(self) -> u32 {
    self.cpu.clock_rate()
  }
}

impl Default for Chip8 {
  fn default() -> Self {
    Chip8::new()
  }
}

impl Chip8 {
  pub fn new() -> Self {
    Chip8 { cpu: Cpu::new() }
  }
}
