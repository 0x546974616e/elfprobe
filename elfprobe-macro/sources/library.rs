#![allow(clippy::needless_pub_self)]

use proc_macro::TokenStream;

mod buffer;
mod cursor;
mod derive;
mod either;
mod entry;
mod literal;
mod parser;
mod rules;
mod token;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  crate::derive::derive(input, "crate::pod::Pod")
}

#[proc_macro]
pub fn is_hex_literal(input: TokenStream) -> TokenStream {
  literal::map_boolean(input, literal::is_hex)
}

#[proc_macro]
pub fn is_bin_literal(input: TokenStream) -> TokenStream {
  literal::map_boolean(input, literal::is_bin)
}

// TODO:
// - upper
// - lower
// - pascal
// - snake
// - camel
// - concat( dada = ...) { fn #dada() {} }
