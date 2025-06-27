use std::fmt::Debug;

use crate::types::{U8, U16, I16, I32, U32, U64, I64, TypeOperation, Endian, EndianInto};
use crate::pod::Pod;

use elfprobe_macro::Pod;

/// 32-bit ELF base types.
/// See `/usr/include{/linux,}/elf.h`
#[allow(non_snake_case)]
pub mod Elf32 {
  use super::*;

  /// Unsigned program address.
  pub type Addr = U32;

  /// Unsigned file offset.
  pub type Off = U32;

  /// Unsigned tiny integer
  pub type Uchar = U8;

  /// Unsigned small integer.
  pub type Half = U16;

  /// Unsigned medium integer.
  pub type Word = U32;

  /// Signed medium integer.
  pub type Sword = I32;
}

/// 64-bit ELF base types.
/// See `/usr/include{/linux,}/elf.h`
#[allow(non_snake_case)]
pub mod Elf64 {
  use super::*;

  /// Unsigned program address.
  pub type Addr = U64;

  /// Unsigned file offset.
  pub type Off = U64;

  /// Unsigned tiny byte.
  pub type Uchar = U8;

  /// Unsigned small integer.
  pub type Half = U16;

  /// Signed small integer.
  pub type SHalf = I16;

  /// Unsigned medium integer.
  pub type Word = U32;

  /// Signed medium integer.
  pub type SWord = I32;

  /// Unsigned large integer.
  pub type XWord = U64;

  /// Signed large integer.
  pub type SXWord = I64;
}

// Trait aliases are still experimental (`trait Bounds = ...`).
macro_rules! make_elftype {
  ($($bounds: tt),+) => {
    pub trait ElfType: Pod {
      /// Unsigned program address.
      type Addr: $($bounds+)+;

      /// Unsigned file offset.
      type Off: $($bounds+)+;

      /// Unsigned tiny integer.
      type Uchar: $($bounds+)+;

      /// Unsigned small integer.
      type Half: $($bounds+)+ EndianInto<usize>;

      /// Unsigned medium integer.
      type Word: $($bounds+)+;

      /// Signed medium integer.
      type Sword: $($bounds+)+;

      /// Unsigned large integer.
      type Xword: $($bounds+)+;
    }
  };
}

make_elftype!(Pod, Debug, Default, TypeOperation);

#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfType32;
impl ElfType for ElfType32 {
  /// Unsigned program address.
  type Addr = Elf32::Addr;

  /// Unsigned file offset.
  type Off = Elf32::Off;

  /// Unsigned tiny integer.
  type Uchar = Elf32::Uchar;

  /// Unsigned small integer.
  type Half = Elf32::Half;

  /// Unsigned medium integer.
  type Word = Elf32::Word;

  /// Signed medium integer.
  type Sword = Elf32::Sword;

  /// Unsigned large integer.
  type Xword = Elf32::Word;
}

#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfType64;
impl ElfType for ElfType64 {
  /// Unsigned program address.
  type Addr = Elf64::Addr;

  /// Unsigned file offset.
  type Off = Elf64::Off;

  /// Unsigned tiny integer.
  type Uchar = Elf64::Uchar;

  /// Unsigned small integer.
  type Half = Elf64::Half;

  /// Unsigned medium integer.
  type Word = Elf64::Word;

  /// Signed medium integer.
  type Sword = Elf64::SWord;

  /// Unsigned large integer.
  type Xword = Elf64::XWord;
}

fn fafa<ElfType: self::ElfType>(value: &mut ElfType::Addr, endian: Endian) {
  // let dada: ElfType::Addr::Type = value.get(endian);
  let dada = value.get(endian);
  let fafa = value.get(endian);
  let haha = dada + fafa;
  value.set(haha, endian);
}

trait Dada<ElfType: self::ElfType> {
  fn dada(value: &mut ElfType::Addr, endian: Endian);
}

impl<ElfType: self::ElfType> Dada<ElfType> for ElfType {
  fn dada(value: &mut ElfType::Addr, endian: Endian) {
    todo!()
  }
}

impl Dada<ElfType64> for ElfType64 {
  fn dada(value: &mut <ElfType64 as ElfType>::Addr, endian: Endian) {
      todo!()
  }
}

impl Dada<ElfType32> for ElfType32 {
  fn dada(value: &mut <ElfType32 as ElfType>::Addr, endian: Endian) {
      todo!()
  }
}

#[test]
fn dada() {
  let endian = Endian::Big;
  let mut dada = U32::new(0x11_22, endian);
  fafa::<ElfType32>(&mut dada, endian);
  assert_eq!(dada.get(endian), 0x11_22 + 0x11_22);
}

fn okokoko<ElfType: self::ElfType>(value: &mut ElfType::Addr) {
  let endian = Endian::Big;
  Dada::<ElfType>::dada(value, endian);
}
