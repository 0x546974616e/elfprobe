use proc_macro::TokenTree;

use crate::entry::Identifier;
use crate::token::Token;

use super::Collect;
use super::Parse;
use super::Stream;

///
/// - [LifetimeParam]: \
///   [LifetimeOrLabel] ( `:` [LifetimeBounds] )?
///
/// [LifetimeParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
///
#[derive(Debug)]
pub(crate) struct LifetimeParam {
  pub(super) lifetime: LifetimeOrLabel,
  pub(super) bounds: Option<(Token![:], LifetimeBounds)>,
}

impl Parse for LifetimeParam {
  fn parse(input: Stream) -> Option<Self> {
    Some(LifetimeParam {
      lifetime: input.parse::<LifetimeOrLabel>()?,
      // Try blocks are still experimental.
      bounds: {
        let ahead = input.fork();
        match (ahead.parse(), ahead.parse()) {
          (Some(colon), Some(bounds)) => {
            input.merge(ahead);
            Some((colon, bounds))
          }
          (Some(_colon), None) => {
            // It seems that Rust accepts a colon alone.
            input.merge(ahead);
            None
          }
          _ => None,
        }
      },
    })
  }
}

impl Collect for LifetimeParam {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.lifetime.collect(tree);
    self.bounds.collect(tree);
  }
}

///
/// - [LifetimeOrLabel] :
///    `'` [NonKeywordIdentifier]
///
/// [LifetimeOrLabel]: https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels
/// [NonKeywordIdentifier]: https://doc.rust-lang.org/reference/identifiers.html
///
#[derive(Debug)]
pub(crate) struct LifetimeOrLabel {
  pub(super) quote: Token![simple_quote],
  pub(super) lifetime: Identifier,
}

impl Parse for LifetimeOrLabel {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork();
    match (ahead.parse(), ahead.parse()) {
      (Some(quote), Some(lifetime)) => {
        input.merge(ahead); // Move the cursor.
        Some(LifetimeOrLabel { quote, lifetime })
      }
      _ => None,
    }
  }
}

impl Collect for LifetimeOrLabel {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.quote.collect(tree);
    self.lifetime.collect(tree);
  }
}

///
/// - [LifetimeBounds] :
///   ( [Lifetime] + )* [Lifetime]?
///
/// [LifetimeBounds]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) struct LifetimeBounds {
  pub(super) lifetimes: Vec<(Lifetime, Option<Token![+]>)>,
}

impl Parse for LifetimeBounds {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork(); // All or nothing.
    let mut lifetimes = Vec::new();

    while let Some(lifetime) = ahead.parse() {
      // Pluses are mandatory, but we don't look at them
      // because we assume the stream is syntactically valid.
      lifetimes.push((lifetime, ahead.parse()));
    }

    if lifetimes.is_empty() {
      return None;
    }

    input.merge(ahead); // Move the cursor.
    Some(LifetimeBounds { lifetimes })
  }
}

impl Collect for LifetimeBounds {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.lifetimes.collect(tree);
  }
}

///
/// - [Lifetime] :
///   [LifetimeOrLabel] | `'static` | `'_`
///
/// [Lifetime]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) enum Lifetime {
  Lifetime(LifetimeOrLabel),
  Inferred((Token![q], Token![_])),
}

impl Parse for Lifetime {
  fn parse(input: Stream) -> Option<Self> {
    if let Some(lifetime) = input.parse() {
      return Some(Lifetime::Lifetime(lifetime));
    }

    let ahead = input.fork();
    match (ahead.parse(), ahead.parse()) {
      (Some(quote), Some(underscore)) => {
        input.merge(ahead); // Move the cursor.
        Some(Lifetime::Inferred((quote, underscore)))
      }
      _ => None,
    }
  }
}

impl Collect for Lifetime {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    match self {
      Self::Lifetime(lifetime) => lifetime.collect(tree),
      Self::Inferred((quote, underscore)) => {
        quote.collect(tree);
        underscore.collect(tree);
      }
    }
  }
}
