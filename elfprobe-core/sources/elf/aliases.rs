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
  pub type Sword<E> = I32<E>;

  /// Unsigned large integer.
  pub type Xword<E> = U64<E>;

  /// Signed large integer.
  pub type Sxword<E> = I64<E>;
}
