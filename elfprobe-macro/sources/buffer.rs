use std::ops::Range;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

use crate::cursor::Cursor;
use crate::entry::Entry;

#[derive(Debug)]
// Highly inspired by `syn`, clever ideas.
pub(crate) struct Buffer {
  // There's no need to do it lazily, as the macro will succeed every time and
  // consume the entire TokenStream.
  entries: Box<[Entry]>,
}

impl From<TokenStream> for Buffer {
  fn from(stream: TokenStream) -> Self {
    let mut entries = Vec::<Entry>::new();
    Buffer::flatten(&mut entries, stream);
    entries.push(Entry::End()); // Last token allowed to be pointed.
    let entries = entries.into_boxed_slice();
    Buffer { entries }
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

  pub(self) fn flatten(entries: &mut Vec<Entry>, stream: TokenStream) {
    for token in stream.into_iter() {
      match token {
        TokenTree::Literal(literal) => entries.push(Entry::Literal(literal)),
        TokenTree::Ident(identifier) => entries.push(Entry::Identifier(identifier)),
        TokenTree::Punct(punctuation) => entries.push(Entry::Punctuation(punctuation)),

        TokenTree::Group(group) => {
          let begin: usize = entries.len();
          entries.push(Entry::End()); // Is going to be replaced.
          Buffer::flatten(entries, group.stream());
          let end: usize = entries.len();

          let offset: isize = (end - begin).try_into().unwrap();
          entries[begin] = Entry::Group(group, offset);
        }
      }
    }
  }
}
