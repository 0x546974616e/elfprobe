#![allow(non_camel_case_types)] // TODO: Temporary?

// https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
// https://rust-lang.github.io/rfcs/1210-impl-specialization.html#the-default-keyword
// https://users.rust-lang.org/t/whats-default-fn/105388/6

use std::fmt::Display;
use std::marker::PhantomData;

use crate::endian::Endianness;
use crate::pod::Pod;
use crate::primitive::{I16, I32, I64, U16, U32, U64};

// 32-bit ELF base types.
// See /usr/include{/linux,}/elf.h
pub type Elf32_Addr<E> = U32<E>;
pub type Elf32_Half<E> = U16<E>;
pub type Elf32_Off<E> = U32<E>;
pub type Elf32_Sword<E> = I32<E>;
pub type Elf32_Word<E> = U32<E>;

// 64-bit ELF base types.
// See /usr/include{/linux,}/elf.h
pub type Elf64_Addr<E> = U64<E>;
pub type Elf64_Half<E> = U16<E>;
pub type Elf64_SHalf<E> = I16<E>;
pub type Elf64_Off<E> = U64<E>;
pub type Elf64_Sword<E> = I32<E>;
pub type Elf64_Word<E> = U32<E>;
pub type Elf64_Xword<E> = U64<E>;
pub type Elf64_Sxword<E> = I64<E>;

////////////////////////////////////////////////////////////////

pub trait ElfType<Endianness: self::Endianness> {
  /// Unsigned program address
  type Addr: Pod;

  /// Unsigned medium integer
  type Half: Pod;

  /// Unsigned file offset
  type Off: Pod;

  /// Signed large integer
  type Sword: Pod;

  /// Unsigned small integer
  type Uchar: Pod;

  /// Unsigned large integer
  type Word: Pod;
}

#[derive(Default)]
pub struct ElfType32<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> ElfType<E> for ElfType32<E> {
  type Addr = Elf32_Addr<E>;
  type Half = Elf32_Half<E>;
  type Off = Elf32_Off<E>;
  type Sword = Elf32_Sword<E>;
  type Uchar = u8; // Unsigned C char
  type Word = Elf32_Word<E>;
}

#[derive(Default)]
pub struct ElfType64<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> ElfType<E> for ElfType64<E> {
  type Addr = Elf64_Addr<E>;
  type Half = Elf64_Half<E>;
  type Off = Elf64_Off<E>;
  type Sword = Elf64_Sword<E>;
  type Uchar = u8; // Unsigned C char
  type Word = Elf64_Word<E>;
}

////////////////////////////////////////////////////////////////

#[repr(C)]
// #[derive(Default)]
pub struct ElfIdentification<Endianness, ElfType>
where
  Endianness: self::Endianness,
  ElfType: self::ElfType<Endianness>,
{
  pub ei_mag0: ElfType::Uchar,
  pub ei_mag1: ElfType::Uchar,
  pub ei_mag2: ElfType::Uchar,
  pub ei_mag3: ElfType::Uchar,
  pub ei_class: ElfType::Uchar,
  pub ei_data: ElfType::Uchar,
  pub ei_version: ElfType::Uchar,
  pub ei_osabi: ElfType::Uchar, // No specified in elf32 but ok.
  pub ei_abiversion: ElfType::Uchar, // No specified in elf32 but ok.
  pub ei_pad: [ElfType::Uchar; 7],
}

#[test]
fn test_elf_identification_memory_size() {
  use crate::endian::{BigEndian, LittleEndian};
  use std::mem::size_of;

  // TODO TMP
  #[rustfmt::skip] assert_eq!(size_of::<ElfIdentification::<BigEndian, ElfType32<BigEndian>>>(), 16, "32-bits");
  #[rustfmt::skip] assert_eq!(size_of::<ElfIdentification::<BigEndian, ElfType64<BigEndian>>>(), 16, "64-bits");
  #[rustfmt::skip] assert_eq!(size_of::<ElfIdentification::<LittleEndian, ElfType32<LittleEndian>>>(), 16, "32-bits");
  #[rustfmt::skip] assert_eq!(size_of::<ElfIdentification::<LittleEndian, ElfType64<LittleEndian>>>(), 16, "64-bits");
}

////////////////////////////////////////////////////////////////

#[repr(C)]
// #[derive(Default)]
pub struct ElfHeader<Endianness, ElfType>
where
  Endianness: self::Endianness,
  ElfType: self::ElfType<Endianness>,
{
  pub e_ident: ElfIdentification<Endianness, ElfType>,
  pub e_type: ElfType::Half,
  pub e_machine: ElfType::Half,
  pub e_version: ElfType::Word,
  pub e_entry: ElfType::Addr,
  pub e_phoff: ElfType::Off,
  pub e_shoff: ElfType::Off,
  pub e_flags: ElfType::Word,
  pub e_ehsize: ElfType::Half,
  pub e_phentsize: ElfType::Half,
  pub e_phnum: ElfType::Half,
  pub e_shentsize: ElfType::Half,
  pub e_shnum: ElfType::Half,
  pub e_shstrndx: ElfType::Half,
}

////////////////////////////////////////////////////////////////
