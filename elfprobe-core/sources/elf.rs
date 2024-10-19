#![allow(non_camel_case_types)] // TODO: Temporary?
#![allow(unused)] // TODO: Temporary

pub mod aliases;
pub mod header;
pub mod identification;
pub mod magic;
pub mod types;

use header::ElfHeader;
use identification::ElfIdentification;
use types::{ElfType, ElfType32, ElfType64};

// https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
// https://rust-lang.github.io/rfcs/1210-impl-specialization.html#the-default-keyword
// https://users.rust-lang.org/t/whats-default-fn/105388/6

use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::core::BytesError;
use crate::core::Pod;
use crate::core::{BigEndian, Endianness, LittleEndian};
use crate::core::{I16, I32, I64, U16, U32, U64};
use elfprobe_macro::Pod;

// ╔═╗┬  ┬┌─┐┌─┐┌─┐┌─┐
// ╠═╣│  │├─┤└─┐├┤ └─┐
// ╩ ╩┴─┘┴┴ ┴└─┘└─┘└─┘

// ╔═╗┌┬┐┬─┐┬ ┬┌─┐┌┬┐
// ╚═╗ │ ├┬┘│ ││   │
// ╚═╝ ┴ ┴└─└─┘└─┘ ┴

// ╔═╗┬┬  ┌─┐
// ╠╣ ││  ├┤
// ╚  ┴┴─┘└─┘

use crate::core::Reader;

#[cfg(any(test, doc, clippy))]
use crate::utils::parse_hex;

// #[derive(Debug)]
pub struct ElfObject<'data, Reader, ElfType>
where
  Reader: self::Reader<'data>,
  ElfType: self::ElfType,
{
  header: &'data ElfHeader<ElfType>,
  // program_header: &'data ElfType::ProgramHeader,
  data: Reader,
}

impl<'data, Reader, ElfType> Debug for ElfObject<'data, Reader, ElfType>
where
  Reader: self::Reader<'data>,
  ElfType: self::ElfType,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    eprintln!("{:#}", self.header);
    f.debug_struct("_ElfFile").field("header", &self.header).finish()
  }
}

impl<'data, Reader, ElfType> ElfObject<'data, Reader, ElfType>
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
  Elf32Be(ElfObject<'data, Reader, ElfType32<BigEndian>>),
  Elf64Be(ElfObject<'data, Reader, ElfType64<BigEndian>>),
  Elf32Le(ElfObject<'data, Reader, ElfType32<LittleEndian>>),
  Elf64Le(ElfObject<'data, Reader, ElfType64<LittleEndian>>),
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
      match *format {
        [1, 1] => Ok(ElfFile::Elf32Le(ElfObject::parse(data)?)),
        [2, 1] => Ok(ElfFile::Elf64Le(ElfObject::parse(data)?)),
        [1, 2] => Ok(ElfFile::Elf32Be(ElfObject::parse(data)?)),
        [2, 2] => Ok(ElfFile::Elf64Be(ElfObject::parse(data)?)),
        _ => Err(BytesError::Empty), // TODO: TMP Err("Bad class/data"),
      }
    }
  }
}

// Program header/table
// Section header/table

#[test]
fn parser() {
  let bytes = parse_hex(
    r"
      7F 'ELF ; Magic
      01 ; ei_class
      02 ; ei_data
      02 ; ei_version
      03 ; ei_osabi
      00 ; ei_abiversion
      00 00 00 00 00 00 00 ; ei_pad

      0003 ; e_type
      003e ; e_machine
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
