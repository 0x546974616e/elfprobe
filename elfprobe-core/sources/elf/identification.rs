use std::fmt;

use elfprobe_macro::Pod;

use crate::utils::define_constants;

use super::magic::Magic;
use super::types::ElfType;

define_constants! {
  ei_class(u8) "Object file classes",
  ELFCLASS32 = 1 "32-bit objects",
  ELFCLASS64 = 2 "64-bit objects",
}

define_constants! {
  ei_data(u8) "Data encodings",
  ELFDATA2LSB = 1 "2's complement, little-endian",
  ELFDATA2MSB = 2 "2's complement, big-endian",
}

define_constants! {
  ei_version(u8) "Object file version.",
  EV_NONE = 0 "Invalid version",
  EV_CURRENT = 1 "Current version",
}

define_constants! {
  ei_osabi(u8) "dada",
  // ELFOSABI_NONE = 0x0 "UNIX System V ABI",
  ELFOSABI_SYSV = 0x0 "UNIX System V ABI",
  ELFOSABI_HPUX = 0x1 "HP-UX",
  ELFOSABI_NETBSD = 0x2 "NetBSD",
  ELFOSABI_GNU = 0x3 "GNU/Linux",
  // ELFOSABI_LINUX = 0x3 "GNU/Linux",
  ELFOSABI_SOLARIS = 0x6 "Sun Solaris",
  ELFOSABI_AIX = 0x7 "IBM AIX",
  ELFOSABI_IRIX = 0x8 "SGI Irix",
  ELFOSABI_FREEBSD = 0x9 "FreeBSD",
  ELFOSABI_TRU64 = 0xa "Compaq TRU64 UNIX",
  ELFOSABI_MODESTO = 0xb "Novell Modesto",
  ELFOSABI_OPENBSD = 0xc "OpenBSD",
  ELFOSABI_ARM_AEABI = 0x40 "ARM EABI",
  ELFOSABI_ARM = 0x61 "ARM",
  ELFOSABI_STANDALONE = 0xff "Standalone (embedded) application",
}

///
/// Identify the file as an ELF object file, and provide information about the
/// data representation of the object file structures. The bytes of this array
/// that have defined meanings are detailed below. The remaining bytes are
/// reserved for future use, and should be set to zero.
///
/// See [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
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

  /// Identifies the class of the object file ([32-bit or 64-bit][ei_class]).
  pub ei_class: ElfType::Uchar,

  /// Specifies the data encoding ([big-endian or little-endian][ei_data]).
  pub ei_data: ElfType::Uchar,

  /// Specifies the [ELF header version number][ei_version].
  pub ei_version: ElfType::Uchar,

  /// Identifies the [operating system and ABI][ei_osabi] for which the object is prepared.
  pub ei_osabi: ElfType::Uchar, // Not specified in Elf32 but ok.

  /// Identifies the version of the ABI for which the object is prepared.
  pub ei_abiversion: ElfType::Uchar, // Not specified in Elf32 but ok.

  /// Unused bytes. These bytes are reserved and set to zero.
  pub ei_pad: [ElfType::Uchar; 7],
}

impl<ElfType: self::ElfType> fmt::Display for ElfIdentification<ElfType> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("ElfIdentification")
      .field("Magic", &Magic::from(self))
      .field("Class", &ei_class::into_constant(self.ei_class))
      .field("Data", &ei_data::into_constant(self.ei_data))
      .field("Version", &ei_version::into_constant(self.ei_version))
      .field("OS/ABI", &ei_osabi::into_constant(self.ei_osabi))
      .field("ABI Version", &self.ei_abiversion)
      .finish()
  }
}

#[test]
fn test_elf_identification_memory_size() {
  use std::mem::size_of;

  use super::types::{ElfType32, ElfType64};
  use crate::core::{BigEndian, LittleEndian};

  type ElfIdentification32<Endianness> = ElfIdentification<ElfType32<Endianness>>;
  type ElfIdentification64<Endianness> = ElfIdentification<ElfType64<Endianness>>;

  assert_eq!(size_of::<ElfIdentification32<BigEndian>>(), 16, "BE 32-bits");
  assert_eq!(size_of::<ElfIdentification64<BigEndian>>(), 16, "BE 64-bits");
  assert_eq!(size_of::<ElfIdentification32<LittleEndian>>(), 16, "LE 32-bits");
  assert_eq!(size_of::<ElfIdentification64<LittleEndian>>(), 16, "LE 64-bits");
}
