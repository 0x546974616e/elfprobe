use std::fmt;
use std::marker::PhantomData;

use crate::endian::EndianOperation;
use crate::endian::Endianness;
use crate::pod::Pod;

// https://rust-exercises.com/100-exercises/04_traits/02_orphan_rule
// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own

macro_rules! create_from_primitive {
  ($name: ident, $type: ident) => {
    #[doc = concat!("An `", stringify!($type), "` wrapper with selectable endianness.")]
    ///
    /// I think it is convenient to bound the `Endianness` to the struct to then
    /// access all struct's method without respecify the generic endianness
    /// every time.
    ///
    // Or unaligned [u8; N]?
    // Zero-cost abstraction
    // transparent
    // partialeq: the operator must be sysmetric and transitive
    // eq is a marker, (used by hash) operator must be reflexive, sysmetric, transitive
    // TODO
    #[rustfmt::skip]
    #[allow(unused)]
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $name<Endianness: self::Endianness>($type, PhantomData<Endianness>);

    // Ensure that the type is POD.
    impl<Endianness: self::Endianness> Pod for $name<Endianness> {}

    impl<Endianness: self::Endianness> From<$type> for $name<Endianness> {
      #[inline]
      fn from(value: $type) -> Self {
        Self(Endianness::write(value), PhantomData)
      }
    }

    impl<Endianness: self::Endianness> fmt::Debug for $name<Endianness> {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.0 or self.get()?
        // Print hexadecimal value and endianness?
        // write!(f, concat!(stringify!($name), "({:x})"), self.get())
        // TODO: It seems that Debug is not handy to deal with all flags (LowerHex, alternate, etc.)
        f.debug_struct(stringify!($name))
          .field(Endianness::long_name(), &self.0)
          .finish()
      }
    }

    impl<Endianness: self::Endianness> $name<Endianness> {
      #[inline]
      #[allow(unused)]
      pub fn get(self) -> $type {
        // Same as Endianness::read(self.0)
        <Endianness as EndianOperation<$type>>::read(self.0)
      }

      #[inline]
      #[allow(unused)]
      pub fn set(&mut self, value: $type) {
        self.0 = Endianness::write(value);
      }
    }
  };
}

create_from_primitive!(I16, i16);
create_from_primitive!(U16, u16);
create_from_primitive!(I32, i32);
create_from_primitive!(U32, u32);
create_from_primitive!(I64, i64);
create_from_primitive!(U64, u64);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::endian::*;

  macro_rules! test_primitive {
    // NOTE: Without std::concat_idents!() it is easier to use modules.
    ($primitive: ident, $type: ident, $value: literal) => {
      mod $type {
        use super::*;
        test_primitive!($primitive, $type, BigEndian, big_endian, $value);
        test_primitive!($primitive, $type, LittleEndian, little_endian, $value);
      }
    };

    ($primitive: ident, $type: ident, $endian: ident, $name: ident, $value: literal) => {
      mod $name {
        use super::*;

        #[test]
        fn get() {
          let value = $primitive::<$endian>::from($value);
          assert_eq!(value.get(), $value);
        }

        #[test]
        fn set() {
          let mut value = $primitive::<$endian>::from(0x0);
          value.set($value);
          assert_eq!(value.get(), $value);
        }

        #[test]
        fn equal() {
          let value1 = $primitive::<$endian>::from($value);
          let value2 = $primitive::<$endian>::from($value);
          assert_eq!(value1, value2);
        }
      }
    };
  }

  test_primitive!(I16, i16, 0x1122);
  test_primitive!(U16, u16, 0x1122);
  test_primitive!(I32, i32, 0x1122_3344);
  test_primitive!(U32, u32, 0x1122_3344);
  test_primitive!(I64, i64, 0x1122_3344_5566_7788);
  test_primitive!(U64, u64, 0x1122_3344_5566_7788);
}
