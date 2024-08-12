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
  /// Returns the endianness long name (capitalized).
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
    "Big-endian"
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
    "Little-endian"
  }

  fn short_name() -> &'static str {
    "LE"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_endianness {
    // It will be so much profitable to use sts::concat_idents!().
    ($function: ident, $endian: ident, $type: ident, $initial: literal) => {
      #[test]
      fn $function() {
        let mut value;
        // The write/read function composition should return the initial value.
        // Native Endian -> Current Endian (maybe no-op) -> Native Endian
        value = <$endian as EndianOperation<$type>>::write($initial);
        value = <$endian as EndianOperation<$type>>::read(value);
        assert_eq!(value, $initial);
      }
    };
  }

  #[rustfmt::skip] test_endianness!(big_endian_i16_write_read, BigEndian, i16, 0x1122);
  #[rustfmt::skip] test_endianness!(big_endian_u16_write_read, BigEndian, u16, 0x1122);
  #[rustfmt::skip] test_endianness!(big_endian_i32_write_read, BigEndian, i32, 0x1122_3344);
  #[rustfmt::skip] test_endianness!(big_endian_u32_write_read, BigEndian, u32, 0x1122_3344);
  #[rustfmt::skip] test_endianness!(big_endian_i64_write_read, BigEndian, i64, 0x1122_3344_5566_7788);
  #[rustfmt::skip] test_endianness!(big_endian_u64_write_read, BigEndian, u64, 0x1122_3344_5566_7788);

  #[rustfmt::skip] test_endianness!(little_endian_i16_write_read, LittleEndian, i16, 0x1122);
  #[rustfmt::skip] test_endianness!(little_endian_u16_write_read, LittleEndian, u16, 0x1122);
  #[rustfmt::skip] test_endianness!(little_endian_i32_write_read, LittleEndian, i32, 0x1122_3344);
  #[rustfmt::skip] test_endianness!(little_endian_u32_write_read, LittleEndian, u32, 0x1122_3344);
  #[rustfmt::skip] test_endianness!(little_endian_i64_write_read, LittleEndian, i64, 0x1122_3344_5566_7788);
  #[rustfmt::skip] test_endianness!(little_endian_u64_write_read, LittleEndian, u64, 0x1122_3344_5566_7788);
}
