//! Printer helpers (adapters)

use std::ascii;
use std::fmt;

use crate::core::Pod;

// ╔╗ ┬ ┬┌┬┐┌─┐┌─┐
// ╠╩╗└┬┘ │ ├┤ └─┐
// ╚═╝ ┴  ┴ └─┘└─┘

#[repr(transparent)]
pub struct Bytes<Type: Pod>(Type);

impl<Type: Pod> From<Type> for Bytes<Type> {
  #[inline(always)]
  fn from(value: Type) -> Self {
    Bytes(value)
  }
}

impl<Type: Pod + fmt::Display> fmt::Display for Bytes<Type> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{} (bytes)", self.0)
  }
}

// ╦ ╦┌─┐─┐┬
// ╠═╣├┤ ┌┼┘
// ╩ ╩└─┘┴└─

#[repr(transparent)]
pub struct Hex<Type: Pod>(Type);

impl<Type: Pod> From<Type> for Hex<Type> {
  #[inline(always)]
  fn from(value: Type) -> Self {
    Hex(value)
  }
}

impl<Type: Pod + fmt::Display + fmt::LowerHex> fmt::Display for Hex<Type> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{:#x}", self.0)
  }
}

// ╔╦╗┌─┐┌─┐┬┌─┐
// ║║║├─┤│ ┬││
// ╩ ╩┴ ┴└─┘┴└─┘

#[repr(transparent)]
/// Magic number wrapper to properly display it.
pub struct Magic([u8; 4]);

impl From<[u8; 4]> for Magic {
  #[inline(always)]
  fn from(value: [u8; 4]) -> Self {
    Magic(value)
  }
}

impl fmt::Display for Magic {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    use fmt::Write;

    let mut last_is_printable = true;
    for (index, byte) in self.0.iter().enumerate() {
      if index != 0 {
        if !last_is_printable || !byte.is_ascii_graphic() {
          formatter.write_char(' ')?;
        }
      }

      write!(formatter, "{}", ascii::escape_default(*byte))?;
      last_is_printable = byte.is_ascii_graphic();
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn magic_one_non_printable() {
    assert_eq!(Magic([0x7F, b'E', b'L', b'F']).to_string(), r"\x7f ELF",)
  }

  #[test]
  fn magic_two_non_printables() {
    assert_eq!(Magic([0x1, b'A', 0x2, b'b']).to_string(), r"\x01 A \x02 b",)
  }

  #[test]
  fn magic_three_non_printables() {
    assert_eq!(Magic([0x1, 0x2, 0x3, b'A']).to_string(), r"\x01 \x02 \x03 A",)
  }
}

// ╔═╗┌─┐┌─┐┌─┐┌─┐┌┬┐
// ║ ║├┤ ├┤ └─┐├┤  │
// ╚═╝└  └  └─┘└─┘ ┴

#[repr(transparent)]
pub struct FileOffset<Type: Pod>(Type);

impl<Type: Pod> From<Type> for FileOffset<Type> {
  #[inline(always)]
  fn from(value: Type) -> Self {
    FileOffset(value)
  }
}

impl<Type: Pod + fmt::Display> fmt::Display for FileOffset<Type> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{} (bytes into file)", self.0)
  }
}
