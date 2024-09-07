use proc_macro::Ident;
use proc_macro::Punct;

// proc_macro::bridge:
// Internal interface for communicating between a proc_macro client (a proc
// macro crate) and a proc_macro server (a compiler front-end).

pub(crate) use proc_macro::Group;
pub(crate) use proc_macro::Literal;

pub(crate) type Identifier = Ident;
pub(crate) type Punctuation = Punct;

#[derive(Debug)]
pub(crate) enum Entry {
  Literal(Literal),
  Identifier(Identifier),
  Punctuation(Punctuation),
  Group(Group, isize),
  End(),
}
