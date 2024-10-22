// #![allow(unused)]

trait Fafa {}
trait Gaga {}
trait Haha {}

use pod as core;
mod pod {
  pub trait Pod {}
  pub fn test(_: impl Pod) {}
}

pub mod a {
  pub trait A {}
  pub trait Foo {
    type I;
  }
  pub mod b {
    pub trait B {}
    pub mod c {
      pub trait C {}
      pub mod d {
        pub trait D {}
      }
    }
  }
}

macro_rules! impl_dada {
  ($($type: ident),*) => {
    $(
      impl Fafa for $type {}
      impl Gaga for $type {}
      impl Haha for $type {}
      impl a::A for $type {}
      impl a::b::B for $type {}
      impl a::b::c::C for $type {}
      impl a::b::c::d::D for $type {}
      impl a::Foo for $type {
        type I = $type;
      }
    )*
  };
}

impl_dada!(u8, u16, u32, u64, u128, usize);
impl_dada!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
  use super::*;
  use elfprobe_macro::Pod;
  use std::marker::PhantomData;

  mod r#struct {
    use super::*;

    #[test]
    fn zst_struct() {
      #[derive(Pod)]
      struct Dada;
      pod::test(Dada);
    }

    #[test]
    fn empty_struct() {
      #[derive(Pod)]
      struct Dada {}
      pod::test(Dada {});
    }

    #[test]
    fn empty_tuple_struct() {
      #[derive(Pod)]
      struct Dada();
      pod::test(Dada());
    }

    #[test]
    fn struct_attributes() {
      #[derive(Pod, Default)]
      #[allow(unused)]
      pub(self) struct Dada {
        a: u128,
        b: u64,
        c: u32,
        d: u16,
        e: u8,
      }
      pod::test(Dada::default());
    }

    #[test]
    fn tuple_struct_parameters() {
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub(super) struct Dada(u128, u64, u32, u16, u8);
      pod::test(Dada::default());
    }

    #[test]
    fn empty_generics() {
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub(super) struct Dada(Option<()>);
      pod::test(Dada::default());
    }
  }

  mod restriction {
    use super::*;

    #[test]
    fn pub_visibility() {
      #[derive(Pod, Default)]
      pub struct Dada();
      pod::test(Dada::default());
    }

    #[test]
    fn self_visibility() {
      #[derive(Pod, Default)]
      pub(self) struct Dada();
      pod::test(Dada::default());
    }

    #[test]
    fn super_visibility() {
      #[derive(Pod, Default)]
      pub(super) struct Dada();
      pod::test(Dada::default());
    }

    #[test]
    fn crate_visibility() {
      #[derive(Pod, Default)]
      pub(crate) struct Dada();
      pod::test(Dada::default());
    }

    #[test]
    fn path_visibility() {
      #[derive(Pod, Default)]
      pub(in crate::tests::restriction) struct Dada;
      pod::test(Dada::default());
    }
  }

  mod attribute {
    use super::*;

    #[test]
    fn one_attribute() {
      #[derive(Pod)]
      #[allow(unused_attributes)]
      pub(super) struct Dada();
      pod::test(Dada());
    }

    #[test]
    fn multiple_attributes() {
      #[derive(Pod, Default)]
      #[allow(unused_attributes)]
      #[allow(dead_code)]
      pub(self) struct Dada {}
      pod::test(Dada::default());
    }
  }

  mod lifetime {
    use super::*;

    #[test]
    fn one_lifetime() {
      #[derive(Pod)]
      pub(self) struct Dada<'a>(PhantomData<&'a ()>);
      pod::test(Dada(PhantomData));
    }

    #[test]
    fn one_lifetime_bound() {
      #[derive(Pod, Default)]
      pub(super) struct Dada<'a: 'static>(PhantomData<&'a ()>);
      pod::test(Dada::default());
    }

    #[test]
    fn two_lifetimes() {
      #[derive(Pod, Default)]
      pub(crate) struct Dada<'a, 'b> {
        a: PhantomData<&'a ()>,
        b: PhantomData<&'b ()>,
      }
      pod::test(Dada::default());
    }

    #[test]
    fn two_lifetimes_bound() {
      #[derive(Pod, Default)]
      pub(crate) struct Dada<'a, 'b: 'a> {
        a: PhantomData<&'a ()>,
        b: PhantomData<&'b ()>,
      }
      pod::test(Dada::default());
    }

    #[test]
    fn multiple_lifetime_bounds() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<'a, 'b: 'a + 'static + 'c +, 'c: 'b + 'a,> {
        a: PhantomData<&'a ()>,
        b: PhantomData<&'b ()>,
        c: PhantomData<&'c ()>,
      }
      pod::test(Dada::default());
    }

    #[test]
    fn trailing_colon() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<'a:>(PhantomData<&'a ()>);
      pod::test(Dada::default());
    }

    #[test]
    fn trailing_comma() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<'a,>(PhantomData<&'a ()>);
      pod::test(Dada::default());
    }

    #[test]
    fn trailing_punctuations() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<'a:, 'b:'a+>{
        a: PhantomData<&'a ()>,
        b: PhantomData<&'b ()>,
      }
      pod::test(Dada::default());
    }
  }

  mod r#trait {
    use super::*;

    #[test]
    fn one_trait() {
      #[derive(Pod)]
      pub(self) struct Dada<A>(A);
      pod::test(Dada(1u8));
    }

    #[test]
    fn one_trait_bound() {
      #[derive(Pod, Default)]
      pub(super) struct Dada<A: Fafa>(A);
      pod::test(Dada(1u16));
    }

    #[test]
    fn two_traits() {
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub(crate) struct Dada<A, B> {
        a: A,
        b: B,
      }
      pod::test(Dada { a: 1u32, b: 2u32 });
    }

    #[test]
    fn two_traits_bound() {
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub(crate) struct Dada<A, B: Fafa> {
        a: A,
        b: B,
      }
      pod::test(Dada { a: 1u64, b: 2i64 });
    }

    #[test]
    fn multiple_trait_bounds() {
      #[rustfmt::skip]
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub struct Dada<A, B: Haha + Fafa +, C: Fafa + Gaga,> {
        a: A,
        b: B,
        c: C,
      }
      pod::test(Dada::<i32, u64, i32>::default());
    }

    #[test]
    fn trailing_colon() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<A:>(A);
      pod::test(Dada(1i8));
    }

    #[test]
    fn trailing_comma() {
      #[rustfmt::skip]
      #[derive(Pod, Default)]
      pub struct Dada<A,>(A);
      pod::test(Dada(1i32));
    }

    #[test]
    fn trailing_punctuations() {
      #[rustfmt::skip]
      #[allow(unused)]
      #[derive(Pod, Default)]
      pub struct Dada<A:, B: Fafa +>{
        a: A,
        b: B,
      }
      pod::test(Dada { a: 0x1u32, b: 0b1i32 });
    }
  }

  mod path {
    use super::*;

    #[test]
    fn one_segment() {
      #[derive(Pod)]
      #[rustfmt::skip]
      pub(in self) struct Dada<A: a::A +,>(A);
      pod::test(Dada(1u8));
    }

    #[test]
    fn two_segments() {
      #[derive(Pod)]
      pub(in super::path) struct Dada<A: a::b::B>(A);
      pod::test(Dada(1u8));
    }

    #[test]
    fn multiple_segments() {
      #[derive(Pod)]
      pub(self) struct Dada<A: a::b::B + a::A + a::Foo, B: a::b::c::d::D>(A, B);
      pod::test(Dada(1u8, 2i16));
    }

    #[test]
    fn trailing_punctuations() {
      #[derive(Pod)]
      #[rustfmt::skip]
      pub struct Dada<A:, B: a::b::B + , C: a::A + a::b::c::C +, >(A, B, C);
      pod::test(Dada(1u8, 2i16, 1u8));
    }
  }

  mod r#where {
    use super::*;

    #[test]
    fn one_where_item() {
      #[derive(Pod, Default)]
      #[allow(dead_code)]
      pub(super) struct Albator<A>
      where
        A: a::A + Default,
      {
        a: A,
      }
      pod::test(Albator::<isize>::default());
    }

    #[test]
    fn multiple_where_items() {
      #[derive(Pod)]
      #[allow(dead_code)]
      pub(super) struct Goldorak<A, B>(A, B)
      where
        A: a::A,
        B: a::b::B + self::Fafa + a::Foo;
      pod::test(Goldorak(1u8, 2i16));
    }
  }

  mod mix {
    use super::*;

    #[test]
    fn everything() {
      trait Here {}
      impl Here for u8 {}

      #[derive(Pod)]
      #[allow(unused)]
      #[repr(packed)]
      pub(in self::super::super::tests::mix) struct Dada<
        'a: 'static + 'static,
        'b: 'static + 'a + 'static,
        A: 'static + self::Fafa + 'a + 'b + Fafa + a::b::c::d::D + crate::Fafa,
        B: self::Gaga + crate::a::b::B,
        C,
        D: Here + 'b + Haha,
        E,
        F,
      >
      where
        E: 'a + a::b::B + 'static,
        F: Default,
      {
        a: PhantomData<&'a B>,
        b: PhantomData<&'b A>,
        c: C,
        d: D,
        e: PhantomData<&'static E>,
        f: F,
      }

      pod::test(Dada {
        a: PhantomData::<&isize>,
        b: PhantomData::<&u128>,
        c: 1isize,
        d: 2u8,
        e: PhantomData::<&i32>,
        f: u64::default(),
      })
    }
  }
}
