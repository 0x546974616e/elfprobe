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

impl fmt::Debug for Magic {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    use fmt::Write;

    let mut string = String::new();
    for byte in self.0 {
      write!(string, "{}", ascii::escape_default(byte));
      if !byte.is_ascii_graphic() {
        string.push(' ');
      }
    }

    formatter.write_str(string.as_str())
  }
}
