use std::ops::Range;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

use crate::cursor::Cursor;
use crate::entry::Entry;

#[derive(Debug)]
// Highly inspired by `syn`, clever ideas.
pub(crate) struct Buffer {
  entries: Box<[Entry]>,
}

impl From<TokenStream> for Buffer {
  fn from(stream: TokenStream) -> Self {
    // There's no need to do it lazily.
    // The macro will succeed every time and consume the entire TokenStream.
    let mut vector: Vec<_> = stream.into_iter().map(Entry::from).collect();
    vector.push(Entry::End()); // Last token allowed to be pointed.
    Buffer {
      entries: vector.into_boxed_slice(),
    }
  }
}

impl Buffer {
  #[inline(always)]
  pub(crate) fn cursor(&self) -> Cursor {
    Cursor::from(self)
  }

  #[inline(always)]
  pub(super) fn as_ptr_range(&self) -> Range<*const Entry> {
    self.entries.as_ptr_range()
  }
}
