use std::ascii;
use std::fmt;

use super::identification::ElfIdentification;
use super::types::ElfType;

/// Magic number wrapper to properly display it.
pub struct Magic([u8; 4]);

impl<ElfType: self::ElfType> From<&ElfIdentification<ElfType>> for Magic {
  fn from(identification: &ElfIdentification<ElfType>) -> Self {
    Self([
      identification.ei_mag0.into(),
      identification.ei_mag1.into(),
      identification.ei_mag2.into(),
      identification.ei_mag3.into(),
    ])
  }
}

impl fmt::Display for Magic {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    use fmt::Write;

    let mut string = String::new();
    let mut last_is_printable = true;
    for byte in self.0 {
      if !last_is_printable || !byte.is_ascii_graphic() {
        string.push(' ');
      }

      write!(string, "{}", ascii::escape_default(byte));
      last_is_printable = byte.is_ascii_graphic();
    }

    formatter.write_str(string.trim_ascii_start())
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
