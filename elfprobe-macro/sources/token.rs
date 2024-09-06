use crate::cursor::Cursor;

use proc_macro::token_stream;
use proc_macro::TokenStream;

// token_stream::IntoIter is implemented with a vector, so cursor simply
// duplicates this vector in the end. But cursor is by design more generic.
pub type TokenCursor = Cursor<token_stream::IntoIter>;

#[allow(unused)]
pub trait FromTokenStream {
  fn from_stream(value: TokenStream) -> Self;
}

impl FromTokenStream for TokenCursor {
  #[inline(always)]
  fn from_stream(stream: TokenStream) -> Self {
    Cursor::from(stream)
  }
}
