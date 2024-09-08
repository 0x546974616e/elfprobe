use proc_macro::TokenTree;
use std::fmt;

use crate::cursor::Cursor;

use crate::entry::Delimiter;
use crate::entry::Group;
use crate::entry::Identifier;
use crate::entry::Punctuation;

use crate::parser::Collect;
use crate::parser::Parse;
use crate::parser::Peek;
use crate::parser::Stream;

macro_rules! create_token {
  (
    struct $name: ident($token: ident) when
      token.$method: ident() is $expr1: expr $(, but $expr2: expr)?
  ) => {
    pub(crate) struct $name {
      // Store a Span instead?
      // Ident::span(), Ident::set_span(), Ident::new()
      // Wrap to_string() result?
      pub token: $token,
    }

    impl From<$token> for $name {
      #[inline(always)]
      fn from(token: $token) -> Self {
        Self { token }
      }
    }

    impl fmt::Debug for $name {
      fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.token.to_string().fmt(formatter)
        // self.token.fmt(formatter)
      }
    }

    impl Collect for $name {
      fn collect(&self, tree: &mut Vec<TokenTree>) {
        tree.push(TokenTree::from(self.token.clone()));
      }
    }

    impl Peek for $name {
      // Does not move the cursor.
      fn peek(stream: Stream) -> bool {
        // match Take::<$token>::entry(stream) {
        match stream.take::<$token>() {
          None => false,
          Some((token, _)) => {
            let value = token.$method();
            value == $expr1 $(&& value != $expr2)?
          }
        }
      }
    }

    impl Parse for $name {
      // Does move the cursor.
      fn parse(stream: Stream) -> Option<Self> {
        let (token, next) = stream.take::<$token>()?;
        let value = token.$method();
        if value == $expr1 $(&& value != $expr2)? {
          stream.merge(next); // Move the cursor.
          Some(Self::from(token.clone()))
        } else { None }
      }
    }

  };
}

// ╦╔═┌─┐┬ ┬┬ ┬┌─┐┬─┐┌┬┐
// ╠╩╗├┤ └┬┘││││ │├┬┘ ││
// ╩ ╩└─┘ ┴ └┴┘└─┘┴└─╶┴┘

macro_rules! define_keywords {
  ($(struct $name: ident = $token: literal)*) => {
    $(
      create_token! {
        struct $name(Identifier) when token.to_string() is $token
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
  ($(struct $name: ident)*) => {
    $(
      create_token! {
        struct $name(Group) when token.delimiter() is Delimiter::$name
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
  ($(struct $name: ident = $punctuation: literal)*) => {
    $(
      create_token! {
        struct $name(Punctuation) when token.as_char() is $punctuation //, but '\''
      }
    )*
  };
}

define_punctuation! {
  struct Colon = ':'
  struct Comma = ','
  struct Dollar = '$'
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

// ╦ ╦┌─┐┬  ┌─┐┌─┐┬─┐
// ╠═╣├┤ │  ├─┘├┤ ├┬┘
// ╩ ╩└─┘┴─┘┴  └─┘┴└─

// Highly inspired by `syn`, clever ideas.
macro_rules! token_helper {
  [#] => { crate::token::Hash };
  [$] => { crate::token::Dollar };
  [+] => { crate::token::Plus };
  [,] => { crate::token::Comma };
  [:] => { crate::token::Colon };
  [;] => { crate::token::SemiColon };
  [<] => { crate::token::Lt };
  [=] => { crate::token::Equals };
  [>] => { crate::token::Gt };
  [?] => { crate::token::Question };
  [_] => { crate::token::Underscore };
  [const] => { crate::token::Const };
  [crate] => { crate::token::Crate };
  [for] => { crate::token::For };
  [pub] => { crate::token::Pub };
  [q] => { crate::token::Quote }; // alias simple_quote
  [simple_quote] => { crate::token::Quote }; // alias '
  [struct] => { crate::token::Struct };
  [where] => { crate::token::Where };
}

macro_rules! group_helper {
  [()] => { crate::token::Parenthesis };
  [[]] => { crate::token::Bracket };
  [{}] => { crate::token::Brace };
}

pub(crate) use group_helper as Group;
pub(crate) use token_helper as Token;
