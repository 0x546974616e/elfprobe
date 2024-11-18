use elfprobe_macro::Pod;

// ╔═╗┌┐┌┌┬┐┬┌─┐┌┐┌
// ║╣ │││ │││├─┤│││
// ╚═╝┘└┘╶┴┘┴┴ ┴┘└┘

#[derive(Copy, Clone)]
pub enum Endian {
  Little,
  Big,
}

pub trait EndianOperation<Runtime, Processor> {
  fn read(self, value: Runtime) -> Processor;
  fn write(self, value: Processor) -> Runtime;
}

macro_rules! impl_endian_operation {
  ($( $type:ty:$bytes:literal ),+) => {
    $(
      impl_endian_operation!(@primitive $type);
      impl_endian_operation!(@array $type:$bytes);
    )+
  };

  (@primitive $type:ty) => {
    impl EndianOperation<$type, $type> for Endian {
      fn read(self, value: $type) -> $type {
        match self {
          Endian::Big => <$type>::from_be(value),
          Endian::Little => <$type>::from_le(value),
        }
      }

      fn write(self, value: $type) -> $type {
        match self {
          Endian::Big => <$type>::to_be(value),
          Endian::Little => <$type>::to_le(value),
        }
      }
    }
  };

  (@array $type:ty:$bytes:literal) => {
    impl EndianOperation<[u8; $bytes], $type> for Endian {
      fn read(self, value: [u8; $bytes]) -> $type {
        match self {
          Endian::Big => <$type>::from_be_bytes(value),
          Endian::Little => <$type>::from_le_bytes(value),
        }
      }

      fn write(self, value: $type) -> [u8; $bytes] {
        match self {
          Endian::Big => <$type>::to_be_bytes(value),
          Endian::Little => <$type>::to_le_bytes(value),
        }
      }
    }
  };
}

impl_endian_operation!(i16:2, u16:2, i32:4, u32:4, i64:8, u64:8);

// ╔╦╗┬ ┬┌─┐┌─┐┌─┐
//  ║ └┬┘├─┘├┤ └─┐
//  ╩  ┴ ┴  └─┘└─┘

macro_rules! create_type {
  (pub $struct:ident($inner:ty) -> $outer:ty) => {
    #[repr(transparent)]
    #[derive(Pod, Copy, Clone, Debug, Default)]
    pub struct $struct($inner);

    impl $struct {
      #[inline(always)]
      pub fn from(value: $outer, endian: Endian) -> Self {
        Self(endian.write(value))
      }

      #[inline(always)]
      pub fn get(self, endian: Endian) -> $outer {
        endian.read(self.0)
      }

      #[inline(always)]
      pub fn set(&mut self, value: $outer, endian: Endian) {
        self.0 = endian.write(value);
      }
    }
  };
}

// #[doc(cfg(not(feature = "unaligned")))]
#[cfg(any(clippy, doc, not(feature = "unaligned")))]
mod aligned {
  use super::*;

  create_type!(pub AlignedI16(i16) -> i16);
  create_type!(pub AlignedU16(u16) -> u16);
  create_type!(pub AlignedU32(u32) -> u32);
  create_type!(pub AlignedI32(i32) -> i32);
  create_type!(pub AlignedI64(i64) -> i64);
  create_type!(pub AlignedU64(u64) -> u64);
}

// #[doc(cfg(feature = "unaligned")]
#[cfg(any(clippy, doc, feature = "unaligned"))]
mod unaligned {
  use super::*;

  create_type!(pub UnalignedI16([u8; 2]) -> i16);
  create_type!(pub UnalignedU16([u8; 2]) -> u16);
  create_type!(pub UnalignedU32([u8; 4]) -> u32);
  create_type!(pub UnalignedI32([u8; 4]) -> i32);
  create_type!(pub UnalignedI64([u8; 8]) -> i64);
  create_type!(pub UnalignedU64([u8; 8]) -> u64);
}

// ╦ ╦┌─┐┌─┐
// ║ ║└─┐├┤
// ╚═╝└─┘└─┘

#[cfg(not(feature = "unaligned"))]
pub use aligned::{
  AlignedI16 as I16, //
  AlignedI32 as I32, //
  AlignedI64 as I64, //
  AlignedU16 as U16, //
  AlignedU32 as U32, //
  AlignedU64 as U64, //
};

#[cfg(feature = "unaligned")]
/// `unaligned` feature is enabled by default.
pub use unaligned::{
  UnalignedI16 as I16, //
  UnalignedI32 as I32, //
  UnalignedI64 as I64, //
  UnalignedU16 as U16, //
  UnalignedU32 as U32, //
  UnalignedU64 as U64, //
};

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_types {
    () => {
      test_types!(@endian big, Big);
      test_types!(@endian little, Little);
    };

    (@endian $module:ident, $endian:ident) => {
      mod $module {
        use super::*;

        test_types!(@type i16, I16, $endian, 0x1122);
        test_types!(@type u16, U16, $endian, 0x1122);
        test_types!(@type i32, I32, $endian, 0x1122_3344);
        test_types!(@type u32, U32, $endian, 0x1122_3344);
        test_types!(@type i64, I64, $endian, 0x1122_3344_5566_7788);
        test_types!(@type u64, U64, $endian, 0x1122_3344_5566_7788);
      }
    };

    (@type $module:ident, $struct:ident, $endian:ident, $initial:literal) => {
      mod $module {
        use super::*;
        use crate::Endian;

        #[test]
        fn get() {
          let endian = Endian::$endian;
          let value = $struct::from($initial, endian);
          assert_eq!(value.get(endian), $initial);
        }

        #[test]
        fn set() {
          let endian = Endian::$endian;
          let mut value = $struct::default();
          value.set($initial, endian);
          assert_eq!(value.get(endian), $initial);
        }
      }
    };
  }

  #[cfg(not(feature = "unaligned"))]
  mod aligned {
    use super::*;
    test_types!();
  }

  #[cfg(feature = "unaligned")]
  mod unaligned {
    use super::*;
    test_types!();
  }
}
