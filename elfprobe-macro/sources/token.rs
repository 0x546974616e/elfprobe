use crate::cursor::SharedCursor;

use proc_macro::token_stream;
use proc_macro::TokenStream;

// token_stream::IntoIter is implemented with a vector, so cursor simply
// duplicates this vector in the end. But cursor is by design more generic.
// pub type TokenCursor = SharedCursor<token_stream::IntoIter>;
pub type TokenCursor = SharedCursor<token_stream::IntoIter>;

#[allow(unused)]
pub trait FromTokenStream {
  fn from_stream(value: TokenStream) -> Self;
}

impl FromTokenStream for TokenCursor {
  #[inline(always)]
  fn from_stream(stream: TokenStream) -> Self {
    SharedCursor::from(stream)
  }
}
