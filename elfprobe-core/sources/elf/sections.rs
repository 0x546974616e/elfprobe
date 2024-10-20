use std::fmt;

use elfprobe_macro::Pod;

use crate::core::Reader;
use crate::utils::{DisplayTable, display_row};

use super::header::ElfHeader;
use super::types::ElfType;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfSection<ElfType: self::ElfType> {
  /// Contains the offset, in bytes, to the section name, relative to the start
  /// of the section name string table.
  sh_name: ElfType::Word,

  /// Identifies the [section type][super::abi::SHType].
  sh_type: ElfType::Word,

  /// Identifies the [attributes][super::abi::SHFlags] of the section.
  sh_flags: ElfType::Xword,

  sh_addr: ElfType::Addr,

  sh_offset: ElfType::Off,

  sh_size: ElfType::Xword,

  sh_link: ElfType::Word,

  sh_info: ElfType::Word,

  sh_addralign: ElfType::Xword,

  sh_entsize: ElfType::Xword,
}

pub struct ElfSectionTable<'data, ElfType: self::ElfType> {
  sections: &'data [ElfSection<ElfType>],
}

impl<'data, ElfType: self::ElfType> ElfSectionTable<'data, ElfType> {
  pub fn parse(data: impl Reader<'data>, header: &ElfHeader<ElfType>) -> Result<Self, ()> {
    use std::mem::size_of;

    let section_size: usize = header.e_shentsize.into();
    if section_size != size_of::<ElfSection<ElfType>>() {
      return Err(())
    }

    // TODO
    Err(())
  }
}

impl<'data, ElfType: self::ElfType> fmt::Display for ElfSectionTable<'data, ElfType> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut table = formatter.display_table("Section Headers:");

    display_row!(table, [ "Nr", "Name", "Type", "Address", "Offset", "Size", "Flags" ]);

    if self.sections.len() == 0 {
      display_row!(table, [ "-", "-", "-", "-", "-", "-", "-" ]);
    }

    for (index, section) in self.sections.iter().enumerate() {
      display_row!(
        table, [
          index,
          section.sh_name,
          section.sh_type,
          section.sh_addr,
          section.sh_offset,
          section.sh_size,
          section.sh_flags,
        ]
      );
    }

    table.finish()
  }
}

#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::ElfSection;
  use crate::core::{BigEndian, LittleEndian};
  use crate::elf::{ElfType32, ElfType64};

  #[test]
  fn size_of_be_32() {
    assert_eq!(size_of::<ElfSection<ElfType32<BigEndian>>>(), 40);
  }

  #[test]
  fn size_of_be_64() {
    assert_eq!(size_of::<ElfSection<ElfType64<BigEndian>>>(), 64);
  }

  #[test]
  fn size_of_le_32() {
    assert_eq!(size_of::<ElfSection<ElfType32<LittleEndian>>>(), 40);
  }

  #[test]
  fn size_of_le_64() {
    assert_eq!(size_of::<ElfSection<ElfType64<LittleEndian>>>(), 64);
  }
}
