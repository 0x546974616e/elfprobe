#![allow(unused)] // TODO: Temporary

mod buffer;
mod cursor;
mod either;
mod entry;
mod parser;
mod token;

use proc_macro::TokenStream;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  // eprintln!("{:#?}", input);
  parse(input);
  TokenStream::new()
}

fn parse(stream: TokenStream) {
  use crate::parser::Peek;
  use crate::token::Token;
  use crate::*;
  let buffer = buffer::Buffer::from(stream);
  eprint!("{:#?}", buffer);
  let cursor = buffer.cursor();
  let token = cursor.parse::<Token![pub]>();
  if <Token![struct]>::peek(&cursor) {
    let token = cursor.parse::<Token![struct]>();
    eprintln!("{:?}", token);
  }
  eprintln!("{:?}", token);
}
