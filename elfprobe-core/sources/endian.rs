///
/// Export functions to read and write primitive value between the implemented
/// endianness and the target processor's endianness.
///
/// The [`'static`][static], [`Copy`] and [`Sized`] trait bounds are here to
/// enforce primitive types (`u8`, `i32`, `u64`...), see [`Pod`](crate::pod::Pod)
/// trait note for more details.
///
/// The initial idea was to use [`std::concat_idents`] to craft method names
/// (`read_u16()`, `write_i32()`...) directly inside the [`Endianness`] trait
/// but it is still a nightly feature (as of August 2024).
///
/// [static]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
///
pub trait EndianOperation<Primitive: 'static + Copy + Sized> {
  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) from the implemented
  /// endianness to the target processor's endianness.
  ///
  fn read(value: Primitive) -> Primitive;

  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) from the target
  /// processor's endianness to the implemented endianness.
  ///
  fn write(value: Primitive) -> Primitive;
}

///
/// Declare required methods to read and write primitive values (`u8`, `i32`,
/// `u64`...) between the implemented endianness and the target processor's
/// endianness at runtime.
///
/// This trait is intended to be used used as a generic type to then detect at
/// runtime which actual endian to specialize (big-endian or little-endian).
///
pub trait Endianness:
  // TODO: Derive from Display and/or Debug?
  'static
  + Copy
  + Eq
  + PartialEq
  + EndianOperation<i16>
  + EndianOperation<u16>
  + EndianOperation<i32>
  + EndianOperation<u32>
  + EndianOperation<i64>
  + EndianOperation<u64>
{
  #[allow(unused)]
  /// Returns the endianness long name (lower case).
  fn long_name() -> &'static str;

  #[allow(unused)]
  /// Returns the endianness short name (capitals).
  fn short_name() -> &'static str;
}

macro_rules! impl_endian_operation {
  // NOTE: Macro can be exported (?):
  // pub(super) use impl_endian_operation;

  ($struct: ident, $endian: literal, $from: ident, $to: ident) => {
    impl_endian_operation!($struct, $endian, i16, $from, $to);
    impl_endian_operation!($struct, $endian, u16, $from, $to);
    impl_endian_operation!($struct, $endian, i32, $from, $to);
    impl_endian_operation!($struct, $endian, u32, $from, $to);
    impl_endian_operation!($struct, $endian, i64, $from, $to);
    impl_endian_operation!($struct, $endian, u64, $from, $to);
  };

  ($struct: ident, $endian: literal, $type: ident, $from: ident, $to: ident) => {
    impl EndianOperation<$type> for $struct {
      #[inline]
      // NOTE: See https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
      #[doc = concat!("Convert an `", stringify!($type), "` value from ", $endian, " endian to the native endian.")]
      fn read(value: $type) -> $type {
        $type::$from(value)
      }

      #[inline]
      // NOTE: The #[doc] attribute can also appear more than once and will be combined.
      #[doc = concat!("Convert an `", stringify!($type), "` value from the native endian to ", $endian, " endian.")]
      fn write(value: $type) -> $type {
        $type::$to(value)
      }
    }
  };
}

///
/// Big endian byte order.
///
/// Define functions to read and write from and to big-endian values at
/// runtime. The `BigEndian` struct is a ZST (Zero-Sized Type) designed to be
/// used with the [`Endianness`] trait as a generic type.
///
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BigEndian;

impl_endian_operation!(BigEndian, "big", from_be, to_be);

impl Endianness for BigEndian {
  fn long_name() -> &'static str {
    "big-endian"
  }

  fn short_name() -> &'static str {
    "BE"
  }
}

///
/// Little endian byte order.
///
/// Define functions to read and write from and to little-endian values at
/// runtime. The `LittleEndian` struct is a ZST (Zero-Sized Type) designed to be
/// used with the [`Endianness`] trait as a generic type.
///
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct LittleEndian;

impl_endian_operation!(LittleEndian, "little", from_le, to_le);

impl Endianness for LittleEndian {
  fn long_name() -> &'static str {
    "little-endian"
  }

  fn short_name() -> &'static str {
    "LE"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_endianness {
    ($endian: ident, $module: ident) => {
      mod $module {
        use super::*;

        // NOTE:
        // Do no use -1 when testing endianness because -1 is a bit sequence of
        // 1 and therefore has no impact on the endianness (same goes for 0).
        test_endianness!($endian, i16, 0x1122);
        test_endianness!($endian, u16, 0x1122);
        test_endianness!($endian, i32, 0x1122_3344);
        test_endianness!($endian, u32, 0x1122_3344);
        test_endianness!($endian, i64, 0x1122_3344_5566_7788);
        test_endianness!($endian, u64, 0x1122_3344_5566_7788);
      }
    };

    // It will be so much profitable to use std::concat_idents!().
    ($endian: ident, $type: ident, $initial: literal) => {
      #[test]
      fn $type() {
        let mut value;
        // The write/read function composition should return the initial value.
        // Native Endian -> Current Endian (maybe no-op) -> Native Endian
        value = <$endian as EndianOperation<$type>>::write($initial);
        value = <$endian as EndianOperation<$type>>::read(value);
        assert_eq!(value, $initial);
      }
    };
  }

  test_endianness!(BigEndian, big_endian);
  test_endianness!(LittleEndian, little_endian);
}
