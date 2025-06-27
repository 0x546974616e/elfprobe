mod pod;
mod reader;
mod types;
mod elf;

pub use pod::{Pod, PodError};
pub use reader::Reader;

pub use types::{Endian, EndianOperation, TypeOperation};
pub use types::{I8, U8, I16, I32, I64, U16, U32, U64};

mod dada {
  use std::ops::{Add, Sub, Mul, Div, Rem};
  // pub trait NumOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }
  trait Num: Sized + Add<Self, Output = Self> { }

/*   macro_rules! dada {
    ($($ty:ty),+ $(,)?) => {
      trait Num:
        'static
        + Sized
        $(+ Add<$ty, Output = Self>)+
      {}

      $(impl Num for $ty {})+
    };
  }

  dada!(
    u8, u16, u32, u64, u128, // usize,
    // i8, i16, i32, i64, i128, // isize,
  );
 */

  trait NumT<T>: Sized + Add<T, Output = T> { }
  impl NumT<usize> for usize {}
  impl Num for usize {}

  trait ElfType {
    // type Addr<T: 'static + Sized>: NumT<T>;
    type Addr: Num;
  }

  struct ElfType32;
  impl ElfType for ElfType32 {
    type Addr = usize;
  }

  fn oaoa<T>(value: T) {

  }

  fn oaoa(value: i8) {

  }

  fn gaga<T: ElfType>(value: T::Addr) {
    value + (1 as isize);
  }

  pub fn multiply_u128<T>(a: T, b: T, result: &mut[u64])  {
    let r = a as u128 * b as u128;
    //fill result with 64 bit chunks
}

  fn fafa<T: Num>(value: T) -> T {
    let a = value + 1;
    return a;
    // let _a = value.add(rhs)
    // T::chec
    // value
  }

  #[test]
  fn dada() {
    // assert_eq!(fafa(1), 2);
    let _a = 1u8 + 3u128;
  }
}
