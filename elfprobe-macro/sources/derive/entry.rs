use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::TokenTree;

// proc_macro::bridge:
// Internal interface for communicating between a proc_macro client (a proc
// macro crate) and a proc_macro server (a compiler front-end).

use super::parser::Collect;
use super::parser::Parse;
use super::parser::Peek;
use super::parser::Stream;

pub(crate) use proc_macro::Delimiter;
pub(crate) use proc_macro::Group;
pub(crate) use proc_macro::Literal;

pub(crate) type Identifier = Ident;
pub(crate) type Punctuation = Punct;

#[derive(Debug)]
pub(crate) enum Entry {
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
        fn collect_into(&self, tree: &mut Vec<TokenTree>) {
          tree.push(TokenTree::from(self.clone()));
        }
      }

      impl Peek for $token {
        fn peek(input: Stream) -> bool {
          input.take::<$token>().is_some()
        }
      }

      impl Parse for $token {
        fn parse(input: Stream) -> Option<Self> {
          let (token, next) = input.take::<$token>()?;
          input.merge(next); // Move the cursor.
          Some(token.clone())
        }
      }
    )*
  };
}

implement_parser!(Identifier, Group, Literal, Punctuation);
