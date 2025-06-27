use elfprobe_macro::Pod;

use super::ElfType;
use super::ElfHeader;

use crate::types::EndianInto;
use crate::Reader;
use crate::Endian;
use crate::TypeOperation;

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
  pub fn parse(data: impl Reader<'data>, endian: Endian, header: &ElfHeader<ElfType>) -> Result<Self, ()> {
    use std::mem::size_of;

    let section_size: usize = header.e_shentsize.endian_into(endian);
    if section_size != size_of::<ElfSection<ElfType>>() {
      return Err(())
    }

    // TODO
    Err(())
  }
}

#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::ElfSection;
  use crate::elf::{ElfType32, ElfType64};

  #[test]
  fn size_of_32() {
    assert_eq!(size_of::<ElfSection<ElfType32>>(), 40);
  }

  #[test]
  fn size_of_64() {
    assert_eq!(size_of::<ElfSection<ElfType64>>(), 64);
  }
}
