use std::fmt;
use std::marker::PhantomData;

use crate::endian::Endianness;
use crate::pod::Pod;

// https://rust-exercises.com/100-exercises/04_traits/02_orphan_rule
// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own

// ╔═╗┬─┐┌─┐┌─┐┌┬┐┌─┐
// ║  ├┬┘├┤ ├─┤ │ ├┤
// ╚═╝┴└─└─┘┴ ┴ ┴ └─┘

macro_rules! create_primitive {
  ($struct: ident, $type: ident) => {
    #[doc = concat!("An `", stringify!($type), "` wrapper with runtime endianness.")]
    ///
    /// I think it is convenient to bound the `Endianness` to the struct to then
    /// access all struct's method without respecify the generic endianness
    /// every time.
    ///
    // Or unaligned [u8; N]?
    // https://doc.rust-lang.org/nightly/cargo/reference/features.html#feature-resolver-version-2
    // Zero-cost abstraction
    // transparent
    // partialeq: the operator must be sysmetric and transitive
    // eq is a marker, (used by hash) operator must be reflexive, sysmetric, transitive
    // TODO
    #[rustfmt::skip]
    #[allow(unused)]
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $struct<Endianness: self::Endianness>($type, PhantomData<Endianness>);

    // Ensure that the type is POD.
    impl<Endianness: self::Endianness> Pod for $struct<Endianness> {}

    impl_primitive_method!($struct, $type);
    impl_primitive_format!($struct);
  };
}

// ╔╦╗┌─┐┌┬┐┬ ┬┌─┐┌┬┐
// ║║║├┤  │ ├─┤│ │ ││
// ╩ ╩└─┘ ┴ ┴ ┴└─┘╶┴┘

macro_rules! impl_primitive_method {
  ($struct: ident, $type: ident) => {
    impl<Endianness: self::Endianness> From<$type> for $struct<Endianness> {
      #[inline]
      fn from(value: $type) -> Self {
        Self(Endianness::write(value), PhantomData)
      }
    }

    impl<Endianness: self::Endianness> $struct<Endianness> {
      #[inline]
      #[allow(unused)]
      pub fn get(self) -> $type {
        Endianness::read(self.0)
      }

      #[inline]
      #[allow(unused)]
      pub fn set(&mut self, value: $type) {
        self.0 = Endianness::write(value);
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

create_primitive!(I16, i16);
create_primitive!(U16, u16);
create_primitive!(I32, i32);
create_primitive!(U32, u32);
create_primitive!(I64, i64);
create_primitive!(U64, u64);

// ╔╦╗┌─┐┌─┐┌┬┐
//  ║ ├┤ └─┐ │
//  ╩ └─┘└─┘ ┴

#[cfg(test)]
mod tests {
  use super::*;
  use crate::endian::*;

  macro_rules! test_primitive {
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

  test_primitive!(BigEndian, big_endian);
  test_primitive!(LittleEndian, little_endian);
}
