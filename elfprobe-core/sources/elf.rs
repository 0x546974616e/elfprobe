pub trait ElfType {
  type Word;
  type Addr;
}

pub struct ElfType32 {}

impl ElfType for ElfType32 {
  type Word = u32;
  type Addr = u32;
}

pub struct ElfType64 {}

impl ElfType for ElfType64 {
  type Word = u32;
  type Addr = u64;
}

#[repr(C)]
pub struct ElfParser<Type: ElfType> {
  e_version: Type::Word,
  e_entry: Type::Addr,
}

impl<Type: ElfType> ElfParser<Type> {
  fn parse(&self) {
    use std::mem::size_of;
    let a = size_of::<Self>();
    println!("{}", a);
  }
}

#[cfg(test)]
mod dada {
  use super::*;
  #[test]
  fn fafa() {
    let e32: ElfParser<ElfType32> = ElfParser {
      e_version: 1,
      e_entry: 2,
    };

    let e64: ElfParser<ElfType64> = ElfParser {
      e_version: 1,
      e_entry: 2,
    };

    e32.parse();
    e64.parse();
  }
}
