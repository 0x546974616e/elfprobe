use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::marker::PhantomData;

use elfprobe_macro::Pod;

use crate::core::Endianness;
use crate::core::Pod;
use crate::core::{I16, I32, I64, U16, U32, U64};

/// 32-bit ELF base types.
/// See `/usr/include{/linux,}/elf.h`
#[allow(non_snake_case)]
pub mod Elf32 {
  use super::*;

  /// Unsigned program address.
  pub type Addr<E> = U32<E>;

  /// Unsigned file offset.
  pub type Off<E> = U32<E>;

  /// Unsigned tiny integer
  pub type Uchar = u8;

  /// Unsigned small integer.
  pub type Half<E> = U16<E>;

  /// Unsigned medium integer.
  pub type Word<E> = U32<E>;

  /// Signed medium integer.
  pub type Sword<E> = I32<E>;
}

/// 64-bit ELF base types.
/// See `/usr/include{/linux,}/elf.h`
#[allow(non_snake_case)]
pub mod Elf64 {
  use super::*;

  /// Unsigned program address.
  pub type Addr<E> = U64<E>;

  /// Unsigned file offset.
  pub type Off<E> = U64<E>;

  /// Unsigned tiny byte.
  pub type Uchar = u8;

  /// Unsigned small integer.
  pub type Half<E> = U16<E>;

  /// Signed small integer.
  pub type SHalf<E> = I16<E>;

  /// Unsigned medium integer.
  pub type Word<E> = U32<E>;

  /// Signed medium integer.
  pub type SWord<E> = I32<E>;

  /// Unsigned large integer.
  pub type XWord<E> = U64<E>;

  /// Signed large integer.
  pub type SXWord<E> = I64<E>;
}

// Trait aliases are still experimental (`trait Bounds = ...`).
macro_rules! make_elftype {
  ($($bounds: tt),+) => {
    pub trait ElfType: Pod + Debug {
      type Endian: self::Endianness;

      /// Unsigned program address.
      type Addr: $($bounds+)+ Into<usize>;

      /// Unsigned file offset.
      type Off: $($bounds+)+ Into<usize>;

      /// Unsigned tiny integer.
      type Uchar: $($bounds+)+ Into<usize> + Into<u8>;

      /// Unsigned small integer.
      type Half: $($bounds+)+ Into<usize> + Into<u16>;

      /// Unsigned medium integer.
      type Word: $($bounds+)+ Into<usize> + Into<u32>;

      /// Signed medium integer.
      type Sword: $($bounds+)+ Into<isize> + Into<i32>;

      /// Unsigned large integer.
      type Xword: $($bounds+)+ Into<usize> + Into<usize>;
    }
  };
}

make_elftype!(Pod, Display, Debug, LowerHex, Default);

#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfType32<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> ElfType for ElfType32<E> {
  type Endian = E;

  /// Unsigned program address.
  type Addr = Elf32::Addr<E>;

  /// Unsigned file offset.
  type Off = Elf32::Off<E>;

  /// Unsigned tiny integer.
  type Uchar = Elf32::Uchar;

  /// Unsigned small integer.
  type Half = Elf32::Half<E>;

  /// Unsigned medium integer.
  type Word = Elf32::Word<E>;

  /// Signed medium integer.
  type Sword = Elf32::Sword<E>;

  /// Unsigned large integer.
  type Xword = Elf32::Word<E>;
}

#[derive(Debug, Default, Copy, Clone, Pod)]
pub struct ElfType64<E: self::Endianness>(PhantomData<E>);
impl<E: self::Endianness> ElfType for ElfType64<E> {
  type Endian = E;

  /// Unsigned program address.
  type Addr = Elf64::Addr<E>;

  /// Unsigned file offset.
  type Off = Elf64::Off<E>;

  /// Unsigned tiny integer.
  type Uchar = Elf64::Uchar;

  /// Unsigned small integer.
  type Half = Elf64::Half<E>;

  /// Unsigned medium integer.
  type Word = Elf64::Word<E>;

  /// Signed medium integer.
  type Sword = Elf64::SWord<E>;

  /// Unsigned large integer.
  type Xword = Elf64::XWord<E>;
}
