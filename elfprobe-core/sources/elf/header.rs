use elfprobe_macro::Pod;

use super::identification::ElfIdentification;
use super::types::ElfType;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfHeader<ElfType: self::ElfType> {
  /// Identify the file as an ELF object file, and provide information
  /// about the data representation of the object file structures.
  pub e_ident: ElfIdentification<ElfType>,

  /// Identifies the [object file type][super::abi::EType].
  pub e_type: ElfType::Half,

  /// Identifies the [target architecture][super::abi::EMachine].
  pub e_machine: ElfType::Half,

  /// Identifies the [version][super::abi::EiVersion] of the object file format
  pub e_version: ElfType::Word,

  /// Contains the virtual address of the program entry point.
  pub e_entry: ElfType::Addr,

  /// Contains the file offset, in bytes, of the program header table.
  pub e_phoff: ElfType::Off,

  /// Contains the file offset, in bytes, of the section header table.
  pub e_shoff: ElfType::Off,

  /// Contains processor-specific flags.
  pub e_flags: ElfType::Word,

  /// Contains the size, in bytes, of the ELF header.
  pub e_ehsize: ElfType::Half,

  /// Contains the size, in bytes, of a program header table entry.
  pub e_phentsize: ElfType::Half,

  /// Contains the number of entries in the program header table.
  pub e_phnum: ElfType::Half,

  /// Contains the size, in bytes, of a section header table entry.
  pub e_shentsize: ElfType::Half,

  /// Contains the number of entries in the section header table.
  pub e_shnum: ElfType::Half,

  // Contains the section header table index of the section containing the section name string table.
  pub e_shstrndx: ElfType::Half,
}

#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::ElfHeader;
  use crate::elf::{ElfType32, ElfType64};

  #[test]
  fn size_of_32() {
    assert_eq!(size_of::<ElfHeader<ElfType32>>(), 52);
  }

  #[test]
  fn size_of_64() {
    assert_eq!(size_of::<ElfHeader<ElfType64>>(), 64);
  }
}
