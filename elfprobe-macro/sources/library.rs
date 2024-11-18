#![allow(clippy::module_inception)]
#![allow(clippy::needless_pub_self)]

pub(self) mod derive;
pub(self) mod parser;

use proc_macro::TokenStream;

#[allow(unused)]
#[cfg(any(doctest, clippy))]
///
/// Error `E0277`: the trait bound `Dada: Pod` is not satisfied
///
/// ```compile_fail,E0277
/// use elfprobe_macro::Pod;
///
/// mod core {
///   pub trait Pod {}
///   pub fn test(_: impl Pod) {}
///   //                  ^^^ required by this bound in `test`
/// }
///
/// struct Dada;
/// core::test(Dada);
/// //         ^^^^ the trait `Pod` is not implemented for `Dada`
/// ```
///
struct PodIsNotImplementedE0277;

///
/// ```
/// use elfprobe_macro::Pod;
/// use std::marker::PhantomData;
///
/// mod core {
///   pub trait Pod {}
///   pub fn test(_: impl Pod) {}
/// }
///
/// trait C {}
/// impl C for i32 {}
///
/// #[derive(Pod, Default)]
/// struct Dada<'a: 'static, A, B: C + Default>
///   where A: Default
/// {
///   marker: PhantomData<&'a A>,
///   a: A,
///   b: B,
/// };
///
/// core::test(Dada {
///   marker: PhantomData,
///   a: u16::default(),
///   b: i32::default(),
/// });
/// ```
///
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  let path = String::from(
    match std::env::var("CARGO_PKG_NAME") {
      Ok(package) if package == "elfprobe-macro" => "", // Ok
      Ok(package) if package == "elfprobe-core" => "crate::",
      _ => "elfprobe_core::",
    }
  );

  crate::derive::derive_empty_trait(input, &(path + "Pod"))
}
