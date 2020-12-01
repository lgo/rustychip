use crate::interface::emulator::Clocked;
use crate::interface::serialization::Savable;

/// Timer provdes the emulation for the Chip8 timer components. The
/// Chip8 contains two timers, the sound and delay timer, which both run
/// at 60hz.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Timer {
  counter: u8,
}

impl Savable for Timer {
  fn save(&self, fh: &mut dyn std::io::Write) -> std::io::Result<()> {
    self.counter.save(fh)?;
    Ok(())
  }
  fn load(&mut self, fh: &mut dyn std::io::Read) -> std::io::Result<()> {
    self.counter.load(fh)?;
    Ok(())
  }
}

impl Clocked for Timer {
  fn clock(&mut self) {
    if self.counter > 0 {
      self.counter -= 1
    }
  }

  fn clock_rate(self) -> u32 {
    60
  }
}

impl Default for Timer {
  fn default() -> Self {
    Timer::new()
  }
}

impl Timer {
  pub fn new() -> Self {
    Timer { counter: 0 }
  }

  pub fn is_zero(self) -> bool {
    self.counter == 0
  }

  pub fn set_counter(&mut self, counter: u8) {
    self.counter = counter
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::interface::serialization;

  #[test]
  fn serialiation_roundtrip_works() -> std::io::Result<()> {
    let buf = &mut Vec::new();
    let timer = &Timer::new();
    timer.save(buf)?;
    let loaded_timer = &serialization::read_value::<Timer>(&mut buf.as_slice())?;
    assert_eq!(timer, loaded_timer);
    Ok(())
  }

  #[test]
  fn serialiation_roundtrip_works_with_adjusted_value() -> std::io::Result<()> {
    let buf = &mut Vec::new();
    let mut timer = Timer::new();
    timer.set_counter(10);
    timer.save(buf)?;
    let loaded_timer = &serialization::read_value::<Timer>(&mut buf.as_slice())?;
    assert_eq!(timer, *loaded_timer);
    assert!(
      !loaded_timer.is_zero(),
      "Expected timer counter to be non-zero but instead got zero.",
    );
    Ok(())
  }

  #[test]
  fn is_zero_works() {
    let mut timer = Timer::new();
    timer.set_counter(0);
    assert!(
      timer.is_zero(),
      "Expected timer counter to be zero, instead got: {}",
      timer.counter
    );
    timer.set_counter(1);
    assert!(
      !timer.is_zero(),
      "Expected timer counter to be non-zero but instead got zero.",
    );
    timer.set_counter(2);
    assert!(
      !timer.is_zero(),
      "Expected timer counter to be non-zero but instead got zero.",
    );
  }

  #[test]
  fn clock_works() {
    let mut timer = Timer::new();
    timer.set_counter(5);

    // Tick the clock 4 times.
    for _ in 0..4 {
      timer.clock();
      assert!(
        !timer.is_zero(),
        "Expected timer counter to be non-zero but instead got zero.",
      );
    }

    // Execute one more clock cycle, which should reach zero.
    timer.clock();
    assert!(
      timer.is_zero(),
      "Expected timer counter to be zero, instead got: {}",
      timer.counter
    );

    // Executing more clock cycles should keep the timer at zero.
    for _ in 0..5 {
      timer.clock();
      assert!(
        timer.is_zero(),
        "Expected timer counter to be zero, instead got: {}",
        timer.counter
      );
    }
  }
}
