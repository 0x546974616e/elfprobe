use proc_macro::TokenTree;

use crate::token::Token;

use super::Collect;
use super::Parse;
use super::Stream;

///
/// - [ConstParam] :
///   `const` [Identifier] `:` [Type] ( `=` [Block] | [Identifier] | -? [Literal] )?
///
/// [Literal]: crate::entry::Literal
/// [Identifier]: crate::entry::Identifier
/// [ConstParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
/// [Block]: https://doc.rust-lang.org/reference/expressions/block-expr.html
/// [Type]: https://doc.rust-lang.org/reference/types.html#type-expressions
///
#[derive(Debug)]
pub(crate) struct ConstParam {
  _todo: (), // TODO
}

impl Parse for ConstParam {
  fn parse(input: Stream) -> Option<Self> {
    if input.parse::<Token![const]>().is_some() {
      todo!("Const parameters are not supported for generic parameters yet.")
    }

    None
  }
}

impl Collect for ConstParam {
  fn collect(&self, _tree: &mut Vec<TokenTree>) {
    todo!()
  }
}
