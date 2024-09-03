use std::fmt::Debug;

// ╔═╗┌─┐┌─┐┬─┐┌─┐┌┬┐┬┌─┐┌┐┌┌─┐
// ║ ║├─┘├┤ ├┬┘├─┤ │ ││ ││││└─┐
// ╚═╝┴  └─┘┴└─┴ ┴ ┴ ┴└─┘┘└┘└─┘

///
/// Export functions to read and write aligned primitive value between the
/// implemented endianness and the target processor's endianness.
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
pub trait AlignedEndianOperation<Primitive: 'static + Copy + Sized> {
  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) from the implemented
  /// endianness to the target processor's endianness.
  ///
  #[allow(unused)]
  fn read(value: Primitive) -> Primitive;

  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) from the target
  /// processor's endianness to the implemented endianness.
  ///
  #[allow(unused)]
  fn write(value: Primitive) -> Primitive;
}

///
/// Export functions to read and write unaligned primitive value (as bytes
/// array) between the implemented endianness and the target processor's
/// endianness.
///
/// Note that unaligned access is not safe. Some architectures (such as x86 and
/// x64) can work with unaligned values (albeit slowly), while others (such as
/// ARM, POWER) cannot. For these reasons, values are given as bytes array.
///
/// See [`AlignedEndianOperation`] for more details.
///
pub trait UnalignedEndianOperation<Primitive: 'static + Copy + Sized, const BYTES: usize> {
  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) as a bytes array from
  /// the implemented endianness to the target processor's endianness.
  ///
  #[allow(unused)]
  fn read(value: [u8; BYTES]) -> Primitive;

  ///
  /// Convert a primitive value (`u8`, `i32`, `u64`...) from the target
  /// processor's endianness to the implemented endianness as a bytes array.
  ///
  #[allow(unused)]
  fn write(value: Primitive) -> [u8; BYTES];
}

// ╔═╗┌┐┌┌┬┐┬┌─┐┌┐┌┌┐┌┌─┐┌─┐┌─┐
// ║╣ │││ │││├─┤││││││├┤ └─┐└─┐
// ╚═╝┘└┘╶┴┘┴┴ ┴┘└┘┘└┘└─┘└─┘└─┘

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
  + Default
  + Debug
  // I'm not particularly fond of this approach.
  + AlignedEndianOperation<i16>
  + AlignedEndianOperation<u16>
  + AlignedEndianOperation<i32>
  + AlignedEndianOperation<u32>
  + AlignedEndianOperation<i64>
  + AlignedEndianOperation<u64>
  + UnalignedEndianOperation<i16, 2>
  + UnalignedEndianOperation<u16, 2>
  + UnalignedEndianOperation<i32, 4>
  + UnalignedEndianOperation<u32, 4>
  + UnalignedEndianOperation<i64, 8>
  + UnalignedEndianOperation<u64, 8>
{
  #[allow(unused)]
  /// Returns the endianness long name (lower case).
  fn long_name() -> &'static str;

  #[allow(unused)]
  /// Returns the endianness short name (capitals).
  fn short_name() -> &'static str;
}

// ╦┌┬┐┌─┐┬  ┌─┐┌┬┐┌─┐┌┐┌┌┬┐┌─┐┌┬┐┬┌─┐┌┐┌┌─┐
// ║│││├─┘│  ├┤ │││├┤ │││ │ ├─┤ │ ││ ││││└─┐
// ╩┴ ┴┴  ┴─┘└─┘┴ ┴└─┘┘└┘ ┴ ┴ ┴ ┴ ┴└─┘┘└┘└─┘

macro_rules! impl_endian_operation {
  ( $operation: ty,
    $struct: ident,
    $endian: literal,
    $impl: ty,
    $target: ident,
    $from: ident,
    $to: ident
  ) => {
    impl $operation for $struct {
      #[inline]
      // NOTE: See https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
      #[doc = concat!("Convert an `", stringify!($impl), "` ", $endian, " endian value")]
      #[doc = concat!("to an `", stringify!($target), "` native endian.")]
      fn read(value: $impl) -> $target {
        $target::$from(value)
      }

      #[inline]
      #[doc = concat!("Convert an `", stringify!($target), "` native endian value")]
      #[doc = concat!("to an `", stringify!($impl), "` ", $endian, " endian.")]
      fn write(value: $target) -> $impl {
        $target::$to(value)
      }
    }
  };
}

#[rustfmt::skip] // TODO: TMP
macro_rules! impl_aligned_endian_operation {
  ($struct: ident, $endian: literal, $from: ident, $to: ident) => {
    impl_aligned_endian_operation!($struct, $endian, i16, $from, $to);
    impl_aligned_endian_operation!($struct, $endian, u16, $from, $to);
    impl_aligned_endian_operation!($struct, $endian, i32, $from, $to);
    impl_aligned_endian_operation!($struct, $endian, u32, $from, $to);
    impl_aligned_endian_operation!($struct, $endian, i64, $from, $to);
    impl_aligned_endian_operation!($struct, $endian, u64, $from, $to);
  };

  ($struct: ident, $endian: literal, $type: ident, $from: ident, $to: ident) => {
    impl_endian_operation!(
      AlignedEndianOperation<$type>,
      $struct, $endian, $type, $type, $from, $to
    );
  };
}

#[rustfmt::skip] // TODO: TMP
macro_rules! impl_unaligned_endian_operation {
  ($struct: ident, $endian: literal, $from: ident, $to: ident) => {
    impl_unaligned_endian_operation!($struct, $endian, i16, 2, $from, $to);
    impl_unaligned_endian_operation!($struct, $endian, u16, 2, $from, $to);
    impl_unaligned_endian_operation!($struct, $endian, i32, 4, $from, $to);
    impl_unaligned_endian_operation!($struct, $endian, u32, 4, $from, $to);
    impl_unaligned_endian_operation!($struct, $endian, i64, 8, $from, $to);
    impl_unaligned_endian_operation!($struct, $endian, u64, 8, $from, $to);
  };

  ($struct: ident, $endian: literal, $type: ident, $bytes: literal, $from: ident, $to: ident) => {
    impl_endian_operation!(
      UnalignedEndianOperation<$type, $bytes>,
      $struct, $endian, [u8; $bytes], $type, $from, $to
    );
  };
}

#[rustfmt::skip] // TODO: TMP
macro_rules! impl_endian_operations {
  // NOTE: Macro can be exported (?):
  // pub(super) use impl_endian_operation;
  ( $struct: ident,
    $endian: literal,
    $from_aligned: ident,
    $from_unaligned: ident,
    $to_aligned: ident,
    $to_unaligned: ident
  ) => {
    // It would have been so much easier with std::concat_idents!()...
    impl_aligned_endian_operation!($struct, $endian, $from_aligned, $to_aligned);
    impl_unaligned_endian_operation!($struct, $endian, $from_unaligned, $to_unaligned);
  };
}

// ╔╗ ┬┌─┐    ┌─┐┌┐┌┌┬┐┬┌─┐┌┐┌
// ╠╩╗││ ┬ ── ├┤ │││ │││├─┤│││
// ╚═╝┴└─┘    └─┘┘└┘╶┴┘┴┴ ┴┘└┘

///
/// Big endian byte order.
///
/// Define functions to read and write from and to big-endian values at
/// runtime. The `BigEndian` struct is a ZST (Zero-Sized Type) designed to be
/// used with the [`Endianness`] trait as a generic type.
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct BigEndian;

impl_endian_operations!(BigEndian, "big", from_be, from_be_bytes, to_be, to_be_bytes);

impl Endianness for BigEndian {
  fn long_name() -> &'static str {
    "big-endian"
  }

  fn short_name() -> &'static str {
    "BE"
  }
}

// ╦  ┬┌┬┐┌┬┐┬  ┌─┐    ┌─┐┌┐┌┌┬┐┬┌─┐┌┐┌
// ║  │ │  │ │  ├┤  ── ├┤ │││ │││├─┤│││
// ╩═╝┴ ┴  ┴ ┴─┘└─┘    └─┘┘└┘╶┴┘┴┴ ┴┘└┘

///
/// Little endian byte order.
///
/// Define functions to read and write from and to little-endian values at
/// runtime. The `LittleEndian` struct is a ZST (Zero-Sized Type) designed to be
/// used with the [`Endianness`] trait as a generic type.
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct LittleEndian;

impl_endian_operations!(LittleEndian, "little", from_le, from_le_bytes, to_le, to_le_bytes);

impl Endianness for LittleEndian {
  fn long_name() -> &'static str {
    "little-endian"
  }

  fn short_name() -> &'static str {
    "LE"
  }
}

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

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
        test_endianness!($endian, i16, 2, 0x1122);
        test_endianness!($endian, u16, 2, 0x1122);
        test_endianness!($endian, i32, 4, 0x1122_3344);
        test_endianness!($endian, u32, 4, 0x1122_3344);
        test_endianness!($endian, i64, 8, 0x1122_3344_5566_7788);
        test_endianness!($endian, u64, 8, 0x1122_3344_5566_7788);
      }
    };

    // It will be so much profitable to use std::concat_idents!().
    ($endian: ident, $type: ident, $bytes: literal, $initial: literal) => {
      mod $type {
        use super::*;

        #[test]
        fn aligned() {
          let mut value;
          // The write/read function composition should return the initial value.
          // Native Endian -> Current Endian (maybe no-op) -> Native Endian
          value = <$endian as AlignedEndianOperation<$type>>::write($initial);
          value = <$endian as AlignedEndianOperation<$type>>::read(value);
          assert_eq!(value, $initial);
        }

        #[test]
        fn unaligned() {
          // The write/read function composition should return the initial value.
          // Native Endian -> Current Endian (maybe no-op) -> Native Endian
          let value = <$endian as UnalignedEndianOperation<$type, $bytes>>::write($initial);
          let value = <$endian as UnalignedEndianOperation<$type, $bytes>>::read(value);
          assert_eq!(value, $initial);
        }
      }
    };
  }

  test_endianness!(BigEndian, big_endian);
  test_endianness!(LittleEndian, little_endian);
}
