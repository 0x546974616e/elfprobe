use std::fmt;

use proc_macro::TokenTree;

use super::Parse;
use super::Stream;

use super::entry::Delimiter;
use super::entry::Entry;
use super::entry::Group;
use super::entry::Identifier;
use super::entry::Punctuation;

use super::collect::Collect;

macro_rules! create_token {
  ( struct $name:ident(token: $token:ident)
      impl token.$method:ident() == $expr:expr
  ) => {
    pub(crate) struct $name {
      token: $token,
    }

    impl From<$token> for $name {
      #[inline(always)]
      fn from(token: $token) -> Self {
        Self { token }
      }
    }

    impl fmt::Debug for $name {
      fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(stringify!($name))
        // self.token.fmt(formatter)
      }
    }

    impl Collect for $name {
      fn collect_into(&self, tokens: &mut Vec<TokenTree>) {
        tokens.push(TokenTree::from(self.token.clone()));
      }
    }

    impl Parse for $name {
      // Does move the cursor.
      fn parse(input: Stream) -> Option<Self> {
        match input.entry() {
          Entry::$token(token) if token.$method() == $expr => {
            input.step(); // Move the cursor.
            return Some(Self::from(token.clone()));
          }
          _ => None,
        }
      }
    }
  };
}

// ╦╔═┌─┐┬ ┬┬ ┬┌─┐┬─┐┌┬┐
// ╠╩╗├┤ └┬┘││││ │├┬┘ ││
// ╩ ╩└─┘ ┴ └┴┘└─┘┴└─╶┴┘

macro_rules! define_keywords {
  ($(struct $name:ident = $token:literal)*) => {
    $(
      create_token! {
        struct $name(token: Identifier)
          impl token.to_string() == $token
      }
    )*
  };
}

define_keywords! {
  struct Const = "const"
  struct Crate = "crate"
  struct For = "for"
  struct Pub = "pub"
  struct Struct = "struct"
  struct Where = "where"
}

// ╔═╗┬─┐┌─┐┬ ┬┌─┐
// ║ ╦├┬┘│ ││ │├─┘
// ╚═╝┴└─└─┘└─┘┴

macro_rules! define_groups {
  ($(struct $name:ident)*) => {
    $(
      create_token! {
        struct $name(token: Group)
          impl token.delimiter() == Delimiter::$name
      }
    )*
  };
}

define_groups! {
  struct Parenthesis
  struct Brace
  struct Bracket
}

// ╔═╗┬ ┬┌┐┌┌─┐┌┬┐┬ ┬┌─┐┌┬┐┬┌─┐┌┐┌
// ╠═╝│ │││││   │ │ │├─┤ │ ││ ││││
// ╩  └─┘┘└┘└─┘ ┴ └─┘┴ ┴ ┴ ┴└─┘┘└┘

macro_rules! define_punctuation {
  ($(struct $name:ident = $punctuation:literal)*) => {
    $(
      create_token! {
        struct $name(token: Punctuation)
          impl token.as_char() == $punctuation
      }
    )*
  };
}

define_punctuation! {
  struct Colon = ':'
  struct Comma = ','
  struct Equals = '='
  struct Gt = '>'
  struct Hash = '#'
  struct Lt = '<'
  struct Plus = '+'
  struct Question = '?'
  struct Quote = '\''
  struct SemiColon = ';'
  struct Underscore = '_'
}
