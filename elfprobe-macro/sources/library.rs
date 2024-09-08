#![allow(clippy::needless_pub_self)]
#![allow(unused)] // TODO: Temporary

use proc_macro::TokenStream;

mod buffer;
mod cursor;
mod derive;
mod either;
mod entry;
mod parser;
mod token;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  crate::derive::derive(input, "crate::pod::Pod")
}
