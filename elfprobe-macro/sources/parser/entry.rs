use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::TokenTree;

pub(crate) use proc_macro::Delimiter;
pub(crate) use proc_macro::Group;
pub(crate) use proc_macro::Literal;

pub(crate) type Identifier = Ident;
pub(crate) type Punctuation = Punct;

use super::collect::Collect;
use super::{Parse, Stream};

pub(super) enum Entry {
  Literal(Literal),
  Identifier(Identifier),
  Punctuation(Punctuation),
  Group(Group),
  End(),
}

impl From<TokenTree> for Entry {
  fn from(token: TokenTree) -> Self {
    match token {
      TokenTree::Group(group) => Entry::Group(group),
      TokenTree::Literal(literal) => Entry::Literal(literal),
      TokenTree::Ident(identifier) => Entry::Identifier(identifier),
      TokenTree::Punct(punctuation) => Entry::Punctuation(punctuation),
    }
  }
}

macro_rules! implement_parser {
  ($($token: ident),*) => {
    $(
      impl Collect for $token {
        fn collect_into(&self, tokens: &mut Vec<TokenTree>) {
          tokens.push(TokenTree::from(self.clone()));
        }
      }

      impl Parse for $token {
        fn parse(input: Stream) -> Option<Self> {
          match input.entry() {
            Entry::$token(token) => {
              input.step(); // Move the cursor.
              return Some(token.clone());
            }
            _ => None
          }
        }
      }
    )*
  };
}

implement_parser!(Identifier, Group, Literal, Punctuation);
