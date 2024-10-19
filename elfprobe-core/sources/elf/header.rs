use std::fmt;

use elfprobe_macro::Pod;

use crate::utils::{display_table, Magic, Hex, Bytes, FileOffset};

use super::constants::{EMachine, EType, EiClass, EiData, EiOsabi, EiVersion};
use super::identification::ElfIdentification;
use super::types::ElfType;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfHeader<ElfType: self::ElfType> {
  /// Identify the file as an ELF object file, and provide information
  /// about the data representation of the object file structures.
  pub e_ident: ElfIdentification<ElfType>,

  /// Identifies the [object file type][e_type].
  pub e_type: ElfType::Half,

  /// Identifies the [target architecture][e_machine].
  pub e_machine: ElfType::Half,

  /// Identifies the [version][e_version] of the object file format
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

impl<ElfType: self::ElfType> fmt::Display for ElfHeader<ElfType> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    display_table!(
      formatter, "ELF Header" =>
      [ "Magic:", Magic::from(&self.e_ident) ],
      [ "Class:", EiClass::from(self.e_ident.ei_class) ],
      [ "Data:", EiData::from(self.e_ident.ei_data) ],
      [ "Version:", EiVersion::from(self.e_ident.ei_version) ],
      [ "OS/ABI:", EiOsabi::from(self.e_ident.ei_osabi) ],
      [ "ABI Version:", self.e_ident.ei_abiversion ],
      [ "Type:", EType::from(self.e_type) ],
      [ "Machine:", EMachine::from(self.e_machine) ],
      [ "Version:", Hex::from(self.e_version) ],
      [ "Entry point address:", Hex::from(self.e_entry) ],
      [ "Start of program headers:", FileOffset::from(self.e_phoff) ],
      [ "Start of section headers:", FileOffset::from(self.e_shoff) ],
      [ "Flags:", Hex::from(self.e_flags) ],
      [ "Size of this header:", Bytes::from(self.e_ehsize) ],
      [ "Size of program headers:", Bytes::from(self.e_phentsize) ],
      [ "Number of program headers:", self.e_phnum ],
      [ "Size of section headers:", Bytes::from(self.e_shentsize) ],
      [ "Number of section headers:", self.e_shnum ],
      [ "Section header string table index:", self.e_shstrndx ],
    )
  }
}
