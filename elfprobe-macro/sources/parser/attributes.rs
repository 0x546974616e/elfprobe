use crate::token::Group;
use crate::token::Token;

use super::Parse;
use super::Stream;

///
/// - [OuterAttribute] :
///   `#` `[` [Attr] `]`
///
/// [OuterAttribute]: https://doc.rust-lang.org/reference/attributes.html
/// [Attr]: https://doc.rust-lang.org/reference/attributes.html
///
#[allow(unused)]
#[derive(Debug)]
pub(crate) struct OuterAttribute {
  hash_token: Token![#],
  attr_group: Group![[]], // TODO: Parse underlying group.
}

impl Parse for OuterAttribute {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork();

    // Early return without moving the cursor.
    let value = Some(OuterAttribute {
      hash_token: ahead.parse()?,
      attr_group: ahead.parse()?,
    });

    input.merge(ahead);
    value
  }
}
