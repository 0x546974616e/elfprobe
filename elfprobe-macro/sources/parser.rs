pub(self) mod buffer;
pub(self) mod collect;
pub(self) mod cursor;
pub(self) mod entry;
pub(self) mod parser;
pub(self) mod r#union;

pub(crate) mod rules;
pub(crate) mod tokens;

pub(crate) use buffer::Buffer;
pub(crate) use collect::Collect;
pub(crate) use cursor::Cursor;
pub(crate) use entry::Identifier;
pub(crate) use r#union::Union;

pub(crate) type Stream<'buffer> = &'buffer Cursor<'buffer>;

pub(crate) trait Parse: Sized {
  // Parses and moves the cursor.
  fn parse(stream: Stream) -> Option<Self>;
}
