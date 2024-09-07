use crate::cursor::Cursor;
use crate::token::Token;

pub(crate) type Stream<'buffer> = &'buffer Cursor<'buffer>;

pub(crate) trait Parse: Sized {
  // Parses and moves the cursor.
  fn parse(stream: Stream) -> Option<Self>;
}

pub(crate) trait Peek {
  // Checks required match, does not move the cursor.
  fn peek(stream: Stream) -> bool;
}

///
/// - Visibility: \
///   `pub` \
///   | `pub` `(` `crate` `)` \
///   | `pub` `(` `self` `)` \
///   | `pub` `(` `super` `)` \
///   | `pub` `(` `in` [SimplePath] `)`
///
/// Source: <https://doc.rust-lang.org/reference/visibility-and-privacy.html>
///
struct Visibility {

}

impl Parse for Visibility {
  fn parse(input: Stream) -> Option<Self> {
    let pub_token = input.parse::<Token![pub]>()?;
    todo!()
  }
}
