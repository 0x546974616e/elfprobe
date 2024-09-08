use crate::token::Group;
use crate::token::Token;

use super::Parse;
use super::Stream;

///
/// - [Visibility] :
///     `pub`
///   | `pub` `(` `crate` `)`
///   | `pub` `(` `self` `)`
///   | `pub` `(` `super` `)`
///   | `pub` `(` `in` [SimplePath] `)`
///
/// [Visibility]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
/// [SimplePath]: https://doc.rust-lang.org/reference/paths.html#simple-paths
///
#[allow(unused)]
#[derive(Debug)]
pub(crate) struct Visibility {
  pub_token: Token![pub],
  path_group: Option<Group![()]>, // TODO: Parse underlying group.
}

impl Parse for Visibility {
  fn parse(input: Stream) -> Option<Self> {
    Some(Visibility {
      pub_token: input.parse()?,
      path_group: input.parse(),
    })
  }
}
