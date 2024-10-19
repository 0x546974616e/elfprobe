use std::fmt;
use std::marker::PhantomData;

use super::endian::Endianness;
use elfprobe_macro::Pod;

// https://rust-exercises.com/100-exercises/04_traits/02_orphan_rule
// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own

// ╔═╗┌┬┐┬─┐┬ ┬┌─┐┌┬┐
// ╚═╗ │ ├┬┘│ ││   │
// ╚═╝ ┴ ┴└─└─┘└─┘ ┴

macro_rules! create_primitive {
  ($struct: ident, $alias: ident, $type: ident, $inner: ty, $into: ty, $operation: ty) => {
    #[doc = concat!("An `", stringify!($inner), "` wrapper with runtime endianness.")]
    ///
    /// It's important that this structure is a zero-cost abstraction of its
    /// original value, firstly because it's a simple wrapper to transform its
    /// internal value according to the given endianness, and secondly because
    /// this structure is going to be built directly from a memory-mapped region
    /// (hence the POD requirement and the transparent representation).
    ///
    /// I think it is convenient to bound the [`Endianness`] to the struct to
    /// then access all struct's method without respecify the generic endianness
    /// every time.
    ///
    /// Note that unaligned access is not safe. Some architectures (such as x86
    /// and x64) can work with unaligned values (albeit slowly), while others
    /// (such as ARM, POWER) cannot. See `unaligned` feature.
    ///
    //
    // DEVELOPER NOTES:
    //
    // [`PartialEq`] requires the type to be sysmetric and transitive, and
    // [`Eq`] (a marker trait) requires the type to be, in addition, reflexive
    // (`PartialEq` is a super-trait of `Eq`):
    //
    // - symmetric: `a == b` implies `b == a`
    // - transitive: `a == b` and `b == c` implies `a == c`
    // - reflexive: `a == a` (remember that `NaN` != `NaN`)
    //
    #[allow(unused)]
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pod)]
    pub struct $struct<Endianness: self::Endianness>($inner, PhantomData<Endianness>);

    #[allow(unused)]
    pub type $alias<Endianness> = $struct<Endianness>;

    impl_primitive_method!($struct, $type, $into, $operation);
    impl_primitive_format!($struct);
  };
}

// ╔╦╗┌─┐┌┬┐┬ ┬┌─┐┌┬┐┌─┐
// ║║║├┤  │ ├─┤│ │ ││└─┐
// ╩ ╩└─┘ ┴ ┴ ┴└─┘╶┴┘└─┘

macro_rules! impl_primitive_method {
  ($struct: ident, $type: ident, $into: ty, $operation: ty) => {
    impl<Endianness: self::Endianness> From<$type> for $struct<Endianness> {
      #[inline(always)]
      fn from(value: $type) -> Self {
        Self(<Endianness as $operation>::write(value), PhantomData)
      }
    }

    impl<Endianness: self::Endianness> From<$struct<Endianness>> for $type {
      #[inline(always)]
      fn from(value: $struct<Endianness>) -> $type {
        <Endianness as $operation>::read(value.0)
      }
    }

    impl<Endianness: self::Endianness> From<$struct<Endianness>> for $into {
      #[inline(always)]
      fn from(value: $struct<Endianness>) -> $into {
        <Endianness as $operation>::read(value.0).try_into().expect(
          concat!("Cannot convert an ", stringify!($struct), " into an `", stringify!($into), "`.")
        )
      }
    }

    impl<Endianness: self::Endianness> $struct<Endianness> {
      #[allow(unused)]
      #[inline(always)]
      pub fn get(self) -> $type {
        <Endianness as $operation>::read(self.0)
      }

      #[allow(unused)]
      #[inline(always)]
      pub fn set(&mut self, value: $type) {
        self.0 = <Endianness as $operation>::write(value);
      }
    }
  };
}

// ╔═╗┌─┐┬─┐┌┬┐┌─┐┌┬┐
// ╠╣ │ │├┬┘│││├─┤ │
// ╚  └─┘┴└─┴ ┴┴ ┴ ┴

macro_rules! impl_primitive_format {
  ($struct: ident) => {
    impl_primitive_format!($struct, Display);
    impl_primitive_format!($struct, LowerHex);
    impl_primitive_format!($struct, UpperHex);
    impl_primitive_format!($struct, Binary);
    impl_primitive_format!($struct, Octal);

    impl<Endianness: self::Endianness> fmt::Debug for $struct<Endianness> {
      // '_ are "elided" lifetimes, i.e. the compiler infers the lifetime.
      fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://stdrs.dev/nightly/x86_64-pc-windows-gnu/src/core/fmt/rt.rs.html#57-64
        const DEBUG_LOWER_HEX: u32 = (1 << 4); //  10000
        const DEBUG_UPPER_HEX: u32 = (1 << 5); // 100000

        #[allow(deprecated)]
        // TODO: Temporary because deprecated.
        // https://doc.rust-lang.org/1.81.0/src/core/fmt/mod.rs.html#1919-1927
        if formatter.flags() & (DEBUG_LOWER_HEX | DEBUG_UPPER_HEX) != 0 {
          return self.get().fmt(formatter);
        }

        // TODO: self.0 when a X/x/o/b formatting is specified.
        formatter
          .debug_tuple(stringify!($struct))
          .field(&Endianness::short_name())
          .field(&self.get())
          .finish()
      }
    }
  };

  ($struct: ident, $trait: ident) => {
    impl<Endianness: self::Endianness> fmt::$trait for $struct<Endianness> {
      fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
      }
    }
  };
}

// ╔═╗┬─┐┌─┐┌─┐┌┬┐┌─┐
// ║  ├┬┘├┤ ├─┤ │ ├┤
// ╚═╝┴└─└─┘┴ ┴ ┴ └─┘

// #[doc(cfg(not(feature = "unaligned")))]
#[cfg(any(clippy, doc, not(feature = "unaligned")))]
mod aligned {
  use super::{super::endian::AlignedEndianOperation, *};

  create_primitive!(AlignedI16, I16, i16, i16, isize, AlignedEndianOperation<i16>);
  create_primitive!(AlignedU16, U16, u16, u16, usize, AlignedEndianOperation<u16>);
  create_primitive!(AlignedI32, I32, i32, i32, isize, AlignedEndianOperation<i32>);
  create_primitive!(AlignedU32, U32, u32, u32, usize, AlignedEndianOperation<u32>);
  create_primitive!(AlignedI64, I64, i64, i64, isize, AlignedEndianOperation<i64>);
  create_primitive!(AlignedU64, U64, u64, u64, usize, AlignedEndianOperation<u64>);
}

// #[doc(cfg(feature = "unaligned")]
#[cfg(any(clippy, doc, feature = "unaligned"))]
mod unaligned {
  use super::{super::endian::UnalignedEndianOperation, *};

  create_primitive!(UnalignedI16, I16, i16, [u8; 2], isize, UnalignedEndianOperation<i16, 2>);
  create_primitive!(UnalignedU16, U16, u16, [u8; 2], usize, UnalignedEndianOperation<u16, 2>);
  create_primitive!(UnalignedI32, I32, i32, [u8; 4], isize, UnalignedEndianOperation<i32, 4>);
  create_primitive!(UnalignedU32, U32, u32, [u8; 4], usize, UnalignedEndianOperation<u32, 4>);
  create_primitive!(UnalignedI64, I64, i64, [u8; 8], isize, UnalignedEndianOperation<i64, 8>);
  create_primitive!(UnalignedU64, U64, u64, [u8; 8], usize, UnalignedEndianOperation<u64, 8>);
}

// ╦ ╦┌─┐┌─┐
// ║ ║└─┐├┤
// ╚═╝└─┘└─┘

#[cfg(not(feature = "unaligned"))]
pub use aligned::{I16, I32, I64, U16, U32, U64};

#[cfg(feature = "unaligned")]
/// `unaligned` feature is enabled by default.
pub use unaligned::{I16, I32, I64, U16, U32, U64};

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::{super::endian::*, *};

  macro_rules! test_primitive {
    () => {
      test_primitive!(BigEndian, big_endian);
      test_primitive!(LittleEndian, little_endian);
    };

    ($endian: ident, $module: ident) => {
      mod $module {
        use super::*;

        test_primitive!($endian, I16, i16, 0x1122);
        test_primitive!($endian, U16, u16, 0x1122);
        test_primitive!($endian, I32, i32, 0x1122_3344);
        test_primitive!($endian, U32, u32, 0x1122_3344);
        test_primitive!($endian, I64, i64, 0x1122_3344_5566_7788);
        test_primitive!($endian, U64, u64, 0x1122_3344_5566_7788);
      }
    };

    ($endian: ident, $struct: ident, $type: ident, $initial: literal) => {
      mod $type {
        use super::*;

        #[test]
        fn get() {
          let value = $struct::<$endian>::from($initial);
          println!("{:?}, {}", value, value); // TODO: TMP
          assert_eq!(value.get(), $initial);
        }

        #[test]
        fn set() {
          let mut value = $struct::<$endian>::from(0x0);
          value.set($initial);
          assert_eq!(value.get(), $initial);
        }

        #[test]
        fn equal() {
          let value1 = $struct::<$endian>::from($initial);
          let value2 = $struct::<$endian>::from($initial);
          assert_eq!(value1, value2);
        }
      }
    };
  }

  #[cfg(not(feature = "unaligned"))]
  mod aligned {
    use super::*;
    test_primitive!();
  }

  #[cfg(feature = "unaligned")]
  mod unaligned {
    use super::*;
    test_primitive!();
  }
}
