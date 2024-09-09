#![allow(clippy::needless_pub_self)]

use parser::Collect;
use proc_macro::TokenStream;

mod buffer;
mod cursor;
mod derive;
mod either;
mod entry;
mod parser;
mod rules;
mod token;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  crate::derive::derive(input, "crate::pod::Pod")
}

use crate::buffer::Buffer;

// #[proc_macro_derive(NewParser)]
#[proc_macro]
pub fn test_macro_parser(stream: TokenStream) -> TokenStream {
  use crate::rules::StructType;
  use crate::parser::Parse;
  let buffer = Buffer::from(stream);
  let input = buffer.cursor();

  let tree = StructType::parse(&input);
  eprintln!("################# {:#?}", tree);
  eprintln!("################# {:#?}", std::mem::size_of::<StructType>());

  let mut vec = Vec::new();
  tree.collect(&mut vec);
  eprint!("#### ({:#?}) ####", vec);

  TokenStream::new()
}
