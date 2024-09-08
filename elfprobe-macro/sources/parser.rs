use crate::cursor::Cursor;
use proc_macro::TokenTree;

pub mod attributes;
pub mod constants;
pub mod generics;
pub mod lifetimes;
pub mod types;
pub mod visibilities;

pub(crate) type Stream<'buffer> = &'buffer Cursor<'buffer>;

pub(crate) trait Parse: Sized {
  // Parses and moves the cursor.
  fn parse(stream: Stream) -> Option<Self>;
}

pub(crate) trait Peek {
  // Checks required match, does not move the cursor.
  fn peek(stream: Stream) -> bool;
}

pub(crate) trait Collect {
  fn collect(&self, tree: &mut Vec<TokenTree>);
}

impl<Type: Collect> Collect for Option<Type> {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    if let Some(value) = self {
      value.collect(tree);
    }
  }
}

impl<Type1: Collect, Type2: Collect> Collect for (Type1, Type2) {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.0.collect(tree);
    self.1.collect(tree);
  }
}

impl<Type: Collect> Collect for Vec<Type> {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    for value in self.iter() {
      value.collect(tree);
    }
  }
}
