#![allow(non_camel_case_types)] // TODO: Temporary?
#![allow(unused)] // TODO: Temporary

// https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
// https://rust-lang.github.io/rfcs/1210-impl-specialization.html#the-default-keyword
// https://users.rust-lang.org/t/whats-default-fn/105388/6

use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::endian::{BigEndian, Endianness, LittleEndian};
use crate::error::BytesError;
use crate::pod::Pod;
use crate::primitive::{I16, I32, I64, U16, U32, U64};

// ╔═╗┬  ┬┌─┐┌─┐┌─┐┌─┐
// ╠═╣│  │├─┤└─┐├┤ └─┐
// ╩ ╩┴─┘┴┴ ┴└─┘└─┘└─┘

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

// ╔╦╗┬ ┬┌─┐┌─┐┌─┐
//  ║ └┬┘├─┘├┤ └─┐
//  ╩  ┴ ┴  └─┘└─┘

// Trait aliases are still experimental.
pub trait Type: Pod + Debug + Default {}
impl<T> Type for T where T: Pod + Debug + Default {}

// pub trait ElfType<Endianness: self::Endianness>: Pod + Default {
pub trait ElfType: Type {
  type Endian: self::Endianness;

  /// Unsigned program address
  type Addr: Type;

  /// Unsigned medium integer
  type Half: Type;

  /// Unsigned file offset
  type Off: Type;

  /// Signed large integer
  type Sword: Type;

  /// Unsigned small integer
  type Uchar: Type;

  /// Unsigned large integer
  type Word: Type;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ElfType32<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> Pod for ElfType32<E> {}
impl<E: self::Endianness> ElfType for ElfType32<E> {
  type Endian = E;
  type Addr = Elf32_Addr<E>;
  type Half = Elf32_Half<E>;
  type Off = Elf32_Off<E>;
  type Sword = Elf32_Sword<E>;
  type Uchar = u8; // Unsigned C char
  type Word = Elf32_Word<E>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ElfType64<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> Pod for ElfType64<E> {}
impl<E: self::Endianness> ElfType for ElfType64<E> {
  type Endian = E;
  type Addr = Elf64_Addr<E>;
  type Half = Elf64_Half<E>;
  type Off = Elf64_Off<E>;
  type Sword = Elf64_Sword<E>;
  type Uchar = u8; // Unsigned C char
  type Word = Elf64_Word<E>;
}

// ╔═╗┌┬┐┬─┐┬ ┬┌─┐┌┬┐
// ╚═╗ │ ├┬┘│ ││   │
// ╚═╝ ┴ ┴└─└─┘└─┘ ┴

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ElfIdentification<ElfType: self::ElfType> {
  pub ei_mag0: ElfType::Uchar,
  pub ei_mag1: ElfType::Uchar,
  pub ei_mag2: ElfType::Uchar,
  pub ei_mag3: ElfType::Uchar,
  pub ei_class: ElfType::Uchar,
  pub ei_data: ElfType::Uchar,
  pub ei_version: ElfType::Uchar,
  pub ei_osabi: ElfType::Uchar,      // No specified in elf32 but ok.
  pub ei_abiversion: ElfType::Uchar, // No specified in elf32 but ok.
  pub ei_pad: [ElfType::Uchar; 7],
}

// Ensure that type is POD.
impl <ElfType: self::ElfType> Pod for ElfIdentification<ElfType> {}

#[test]
fn test_elf_identification_memory_size() {
  use crate::endian::{BigEndian, LittleEndian};
  use std::mem::size_of;

  type ElfIdentification32<Endianness> = ElfIdentification<ElfType32<Endianness>>;
  type ElfIdentification64<Endianness> = ElfIdentification<ElfType64<Endianness>>;

  assert_eq!(size_of::<ElfIdentification32<BigEndian>>(), 16, "BE 32-bits");
  assert_eq!(size_of::<ElfIdentification64<BigEndian>>(), 16, "BE 64-bits");
  assert_eq!(size_of::<ElfIdentification32<LittleEndian>>(), 16, "LE 32-bits");
  assert_eq!(size_of::<ElfIdentification64<LittleEndian>>(), 16, "LE 64-bits");
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ElfHeader<ElfType: self::ElfType> {
  pub e_ident: ElfIdentification<ElfType>,
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

// Ensure that type is POD.
impl <ElfType: self::ElfType> Pod for ElfHeader<ElfType> {}

// ╔═╗┬┬  ┌─┐
// ╠╣ ││  ├┤
// ╚  ┴┴─┘└─┘

use crate::reader::Reader;

#[cfg(any(test, doc, clippy))]
use crate::hex::hex;

// #[derive(Debug)]
pub struct _ElfFile<'data, Reader, ElfType>
where
  Reader: self::Reader<'data>,
  ElfType: self::ElfType,
{
  header: &'data ElfHeader<ElfType>,
  // program_header: &'data ElfType::ProgramHeader,
  data: Reader,
}

impl<'data, Reader, ElfType> Debug for _ElfFile<'data, Reader, ElfType>
where
  Reader: self::Reader<'data>,
  ElfType: self::ElfType,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("_ElfFile").field("header", &self.header).finish()
  }
}

impl<'data, Reader, ElfType> _ElfFile<'data, Reader, ElfType>
where
  Reader: self::Reader<'data>,
  ElfType: self::ElfType,
{
  fn parse(data: Reader) -> Result<Self, BytesError> {
    let header = data.read_pod::<ElfHeader<ElfType>>(0)?;
    Ok(Self { header, data })
  }
}

#[derive(Debug)]
pub enum ElfFile<'data, Reader: self::Reader<'data>> {
  Elf32Be(_ElfFile<'data, Reader, ElfType32<BigEndian>>),
  Elf64Be(_ElfFile<'data, Reader, ElfType64<BigEndian>>),
  Elf32Le(_ElfFile<'data, Reader, ElfType32<LittleEndian>>),
  Elf64Le(_ElfFile<'data, Reader, ElfType64<LittleEndian>>),
}

#[allow(unused)]
pub fn parse_elf<'data, Reader>(data: Reader) -> Result<ElfFile<'data, Reader>, BytesError>
where
  Reader: self::Reader<'data>,
{
  let magic = data.read_bytes(4, 0);
  if magic != Some(&[0x7f, b'E', b'L', b'F']) {
    return Err(BytesError::Empty); // TODO: TMP Err("Bad magic");
  }

  match data.read_bytes(2, 4) {
    None => Err(BytesError::Empty), // TODO: TMP Err("No class/data"),
    Some(format) => {
      match format {
        &[1, 1] => Ok(ElfFile::Elf32Le(_ElfFile::parse(data)?)),
        &[2, 1] => Ok(ElfFile::Elf64Le(_ElfFile::parse(data)?)),
        &[1, 2] => Ok(ElfFile::Elf32Be(_ElfFile::parse(data)?)),
        &[2, 2] => Ok(ElfFile::Elf64Be(_ElfFile::parse(data)?)),
        _ => Err(BytesError::Empty), // TODO: TMP Err("Bad class/data"),
      }
    }
  }
}

// Program header/table
// Section header/table

#[test]
fn parser() {
  let bytes = hex(
    r"
      7F 'ELF ; Magic
      01 ; ei_class
      02 ; ei_data
      00 ; ei_version
      00 ; ei_osabi
      00 ; ei_abiversion
      00 00 00 00 00 00 00 ; ei_pad

      0102 ; e_type
      0102 ; e_machine
      0102 0304 ; e_version
      0102 0304 ; e_entry
      0102 0304 ; e_phoff
      0102 0304 ; e_shoff
      0102 0304 ; e_flags
      0102 ; e_ehsize
      0102 ; e_phentsize
      0102 ; e_phnum
      0102 ; e_shentsize
      0102 ; e_shnum
      0102 ; e_shstrndx
    ",
  );

  let binding = bytes.unwrap();
  let result = parse_elf(binding.as_slice());

  println!("{:#x?}", result);
}
