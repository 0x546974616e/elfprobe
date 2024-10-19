use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use elfprobe_macro::Pod;

use crate::core::Endianness;
use crate::core::Pod;

use super::aliases::{Elf32, Elf64};

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
      type Uchar: $($bounds+)+ Into<u8>;

      /// Unsigned small integer.
      type Half: $($bounds+)+ Into<u16>;

      /// Unsigned medium integer.
      type Word: $($bounds+)+ Into<u32>;

      /// Signed medium integer.
      type Sword: $($bounds+)+ Into<i32>;
    }
  };
}

make_elftype!(Pod, Display, Debug, Default);

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
  type Sword = Elf64::Sword<E>;
}
