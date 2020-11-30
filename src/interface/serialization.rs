//! Serialization provides interfaces for the basic serialization of
//! emulator state.
//!
//! Primitve serialization implementations are provided by this module.
//!
//! The interface and implementation is derived from
//! https://github.com/MichaelBurge/nes-emulator
//!
//! Original author: Michael Burge <michaelburge@pobox.com>
//!
//! TODO(joey): Remove once fully implemented.
#![allow(dead_code)]

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub trait Savable {
  #[must_use]
  fn save(&self, fh: &mut dyn Write) -> Result<()>;
  #[must_use]
  fn load(&mut self, fh: &mut dyn Read) -> Result<()>;
}

impl Savable for bool {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let bytes = [*self as u8];
    fh.write_all(&bytes)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut bytes = [0];
    fh.read_exact(&mut bytes)?;
    *self = bytes[0] > 0;
    Ok(())
  }
}

impl Savable for u8 {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let bytes = [*self as u8];
    fh.write_all(&bytes)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut bytes = [0];
    fh.read_exact(&mut bytes)?;
    *self = bytes[0];
    Ok(())
  }
}

impl Savable for u16 {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let bytes = [(*self & 0xff) as u8, ((*self >> 8) & 0xff) as u8];
    fh.write_all(&bytes)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut bytes = [0; 2];
    fh.read_exact(&mut bytes)?;
    *self = 0;
    *self |= bytes[0] as u16;
    *self |= (bytes[1] as u16) << 8;
    Ok(())
  }
}

impl Savable for u32 {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let bytes = [
      ((*self >> 0) & 0xff) as u8,
      ((*self >> 8) & 0xff) as u8,
      ((*self >> 16) & 0xff) as u8,
      ((*self >> 24) & 0xff) as u8,
    ];
    fh.write_all(&bytes)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut bytes = [0u8; 4];
    fh.read_exact(&mut bytes)?;
    *self = 0;
    *self |= (bytes[0] as u32) << 0;
    *self |= (bytes[1] as u32) << 8;
    *self |= (bytes[2] as u32) << 16;
    *self |= (bytes[3] as u32) << 24;
    Ok(())
  }
}

impl Savable for u64 {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let bytes = [
      ((*self >> 0) & 0xff) as u8,
      ((*self >> 8) & 0xff) as u8,
      ((*self >> 16) & 0xff) as u8,
      ((*self >> 24) & 0xff) as u8,
      ((*self >> 32) & 0xff) as u8,
      ((*self >> 40) & 0xff) as u8,
      ((*self >> 48) & 0xff) as u8,
      ((*self >> 56) & 0xff) as u8,
    ];
    fh.write_all(&bytes)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut bytes = [0u8; 8];
    fh.read_exact(&mut bytes)?;
    *self = 0;
    *self |= (bytes[0] as u64) << 0;
    *self |= (bytes[1] as u64) << 8;
    *self |= (bytes[2] as u64) << 16;
    *self |= (bytes[3] as u64) << 24;
    *self |= (bytes[4] as u64) << 32;
    *self |= (bytes[5] as u64) << 40;
    *self |= (bytes[6] as u64) << 48;
    *self |= (bytes[7] as u64) << 56;
    Ok(())
  }
}

impl Savable for usize {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    (*self as u64).save(fh)
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut x: u64 = *self as u64;
    x.load(fh)?;
    *self = x as usize;

    Ok(())
  }
}

impl<T: Savable + Default> Savable for Vec<T> {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let len: usize = self.len();
    len.save(fh)?;
    for i in self.iter() {
      i.save(fh)?;
    }

    Ok(())
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let mut len = 0usize;
    len.load(fh)?;
    self.truncate(0);
    self.reserve(len);
    for _ in 0..len {
      let mut x: T = Default::default();
      x.load(fh)?;
      self.push(x);
    }

    Ok(())
  }
}

impl Savable for String {
  fn save(&self, fh: &mut dyn Write) -> Result<()> {
    let len = self.len() as u32;
    len.save(fh)?;
    for byte in self.bytes() {
      byte.save(fh)?;
    }

    Ok(())
  }
  fn load(&mut self, fh: &mut dyn Read) -> Result<()> {
    let len = read_value::<u32>(fh)? as usize;

    let mut bytes = vec![0; len];
    for i in 0..len {
      bytes[i] = read_value::<u8>(fh)?;
    }
    *self = String::from_utf8(bytes).expect("Invalid utf8");

    Ok(())
  }
}

pub fn read_value<T: Default + Savable>(fh: &mut dyn Read) -> Result<T> {
  let mut t: T = Default::default();
  t.load(fh).map(|_| t)
}

use std::io::Seek;
use std::io::SeekFrom;

pub fn file_position(fh: &mut File) -> u64 {
  fh.seek(SeekFrom::Current(0)).unwrap()
}

#[cfg(test)]
mod tests {

  use super::*;

  // round_trip will do a round-trip serialization of `value`.
  //
  // TODO(joey): I do not know enough Rust to make this method work for
  // Vec, because the trait implementation is by-value but the method
  // implementations (load/save) are over `&mut self`, which is
  // contradictory to a by-reference implementation.
  fn round_trip<T: Default + Savable>(value: T) -> Result<T> {
    let buf = &mut Vec::new();
    // Save the value to the buffer.
    value.save(buf)?;
    // And subsequently read it out.
    let val = read_value::<T>(&mut buf.as_slice())?;
    Ok(val)
  }

  #[test]
  fn savable_bool_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(true)?, true);
    assert_eq!(tests::round_trip(false)?, false);
    Ok(())
  }

  #[test]
  fn savable_u8_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(0u8)?, 0u8);
    assert_eq!(tests::round_trip(1u8)?, 1u8);
    assert_eq!(tests::round_trip(255u8)?, 255u8);
    Ok(())
  }

  #[test]
  fn savable_u16_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(0u16)?, 0u16);
    assert_eq!(tests::round_trip(1u16)?, 1u16);
    assert_eq!(tests::round_trip(65535u16)?, 65535u16);
    Ok(())
  }

  #[test]
  fn savable_u32_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(0u32)?, 0u32);
    assert_eq!(tests::round_trip(1u32)?, 1u32);
    assert_eq!(tests::round_trip(4294967295u32)?, 4294967295u32);
    Ok(())
  }

  #[test]
  fn savable_u64_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(0u64)?, 0u64);
    assert_eq!(tests::round_trip(1u64)?, 1u64);
    assert_eq!(
      tests::round_trip(18446744073709551615u64)?,
      18446744073709551615u64
    );
    Ok(())
  }

  #[test]
  fn savable_usize_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(0usize)?, 0usize);
    assert_eq!(tests::round_trip(1usize)?, 1usize);
    assert_eq!(tests::round_trip(123usize)?, 123usize);
    Ok(())
  }

  #[test]
  fn savable_vec_with_values_roundtrips() -> Result<()> {
    let case = vec![1u8, 2u8, 3u8];
    let buf = &mut Vec::new();
    // Save the vector.
    case.save(buf)?;
    // Read the vector value.
    let result: Vec<u8> = read_value::<Vec<u8>>(&mut buf.as_slice())?;
    // Compare the results.
    assert_eq!(result, case);

    Ok(())
  }
  #[test]
  fn savable_vec_with_empty_roundtrips() -> Result<()> {
    let case = vec![];
    let buf = &mut Vec::new();
    // Save the vector.
    case.save(buf)?;
    // Read the vector value.
    let result: Vec<u8> = read_value::<Vec<u8>>(&mut buf.as_slice())?;

    // Compare the results.
    assert_eq!(result, case);

    Ok(())
  }

  #[test]
  fn savable_str_roundtrips() -> Result<()> {
    assert_eq!(tests::round_trip(String::from(""))?, String::from(""));
    assert_eq!(
      tests::round_trip(String::from("my_test_case"))?,
      String::from("my_test_case")
    );
    Ok(())
  }
}
