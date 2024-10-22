use proc_macro::TokenStream;
use std::ops::Range;

use super::cursor::Cursor;
use super::entry::Entry;

pub(crate) struct Buffer {
  entries: Box<[Entry]>,
}

impl From<TokenStream> for Buffer {
  fn from(stream: TokenStream) -> Self {
    // There is no need to do it lazily.
    // The macro is supposed to succeed every time and consume the entire stream.
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
