use crate::cursor::Cursor;
use crate::entry::Identifier;
use crate::parser::Parse;
use crate::parser::Peek;
use crate::parser::Stream;

macro_rules! define_keywords {
  ($(struct $name: ident $token: literal)*) => {
    $(
      #[derive(Debug)]
      pub(crate) struct $name {
        pub identifier: Identifier,
      }

      impl From<Identifier> for $name {
        fn from(identifier: Identifier) -> Self {
          Self { identifier }
        }
      }

      impl Peek for $name {
        // Does not move the cursor.
        fn peek(stream: Stream) -> bool {
          match stream.identifier() {
            None => false,
            Some((identifier, _)) => {
              // TODO: Wrap token to cache to_string() result?
              identifier.to_string() == $token
            }
          }
        }
      }

      impl Parse for $name {
        // Does move the cursor.
        fn parse(stream: Stream) -> Option<Self> {
          match stream.identifier() {
            Some((identifier, next)) if identifier.to_string() == $token => {
              stream.merge(next); // Move the cursor.
              Some(Self::from(identifier.clone()))
            }
            _ => None
          }
        }
      }
    )*
  };
}

define_keywords! {
  struct Crate "crate"
  struct Enum "enum"
  struct Pub "pub"
  struct SelfType "Self"
  struct SelfValue "self"
  struct Struct "struct"
  struct Super "super"
}

// Highly inspired by `syn`, clever ideas.
macro_rules! Token {
  [crate] => { $crate::token::Crate };
  [enum] => { $crate::token::Enum };
  [pub] => { $crate::token::Pub };
  [Self] => { $crate::token::SelfType };
  [self] => { $crate::token::SelfValue };
  [struct] => { $crate::token::Struct };
  [super] => { $crate::token::Super };
}

pub(crate) use Token;
