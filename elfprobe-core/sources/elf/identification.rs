use std::fmt;

use elfprobe_macro::Pod;

use crate::utils::Magic;

use super::types::ElfType;

///
/// Identify the file as an ELF object file, and provide information about the
/// data representation of the object file structures. The bytes of this array
/// that have defined meanings are detailed below. The remaining bytes are
/// reserved for future use, and should be set to zero.
///
/// See [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf).
///
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfIdentification<ElfType: self::ElfType> {
  /// Magic number byte 1 `0x7F`.
  pub ei_mag0: ElfType::Uchar,

  /// Magic number byte 2 `0x45` (`E`).
  pub ei_mag1: ElfType::Uchar,

  /// Magic number byte 2 `0x4C` (`L`).
  pub ei_mag2: ElfType::Uchar,

  /// Magic number byte 2 `0x46` (`F`).
  pub ei_mag3: ElfType::Uchar,

  /// Identifies the class of the object file ([32-bit or 64-bit][super::abi::EiClass]).
  pub ei_class: ElfType::Uchar,

  /// Specifies the data encoding ([big-endian or little-endian][super::abi::EiData]).
  pub ei_data: ElfType::Uchar,

  /// Specifies the [ELF header version number][super::abi::EiVersion].
  pub ei_version: ElfType::Uchar,

  /// Identifies the [operating system and ABI][super::abi::EiOsabi] for which the object is prepared.
  pub ei_osabi: ElfType::Uchar, // Not specified in Elf32 but ok.

  /// Identifies the version of the ABI for which the object is prepared.
  pub ei_abiversion: ElfType::Uchar, // Not specified in Elf32 but ok.

  /// Unused bytes. These bytes are reserved and set to zero.
  pub ei_pad: [ElfType::Uchar; 7],
}

impl<ElfType: self::ElfType> From<&ElfIdentification<ElfType>> for Magic {
  fn from(identification: &ElfIdentification<ElfType>) -> Magic {
    Magic::from([
      identification.ei_mag0.into(),
      identification.ei_mag1.into(),
      identification.ei_mag2.into(),
      identification.ei_mag3.into(),
    ])
  }
}

#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::ElfIdentification;
  use crate::core::{BigEndian, LittleEndian};
  use crate::elf::{ElfType32, ElfType64};

  #[test]
  fn size_of_be_32() {
    assert_eq!(size_of::<ElfIdentification<ElfType32<BigEndian>>>(), 16);
  }

  #[test]
  fn size_of_be_64() {
    assert_eq!(size_of::<ElfIdentification<ElfType64<BigEndian>>>(), 16);
  }

  #[test]
  fn size_of_le_32() {
    assert_eq!(size_of::<ElfIdentification<ElfType32<LittleEndian>>>(), 16);
  }

  #[test]
  fn size_of_le_64() {
    assert_eq!(size_of::<ElfIdentification<ElfType64<LittleEndian>>>(), 16);
  }
}
