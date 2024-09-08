use proc_macro::TokenTree;

use crate::entry::Identifier;
use crate::token::Group;
use crate::token::Token;

use super::lifetimes::Lifetime;
use super::Collect;
use super::Parse;
use super::Stream;

///
/// - [TypeParam] :
///   [Identifier] ( `:` [TypeParamBounds]? )? ( `=` [Type] )?
///
/// [TypeParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
/// [Type]: https://doc.rust-lang.org/reference/types.html#type-expressions
///
#[derive(Debug)]
pub(crate) struct TypeParam {
  pub(super) identifier: Identifier,
  pub(super) bounds: Option<(Token![:], Option<TypeParamBounds>)>,
}

impl Parse for TypeParam {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork(); // All or nothing.
    let identifier = ahead.parse::<Identifier>()?;

    let bounds = ahead.parse().map(|colon| (colon, ahead.parse()));

    if ahead.parse::<Token![=]>().is_some() {
      panic!("Default generic values are not supported yet.");
    }

    input.merge(ahead); // Move cursor.
    Some(TypeParam { identifier, bounds })
  }
}

impl Collect for TypeParam {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.identifier.collect(tree);
    self.bounds.collect(tree);
  }
}

///
/// - [TypeParamBounds] :
///   [TypeParamBound] ( `+` [TypeParamBound] )* `+`?
///
/// [TypeParamBounds]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) struct TypeParamBounds {
  pub(super) bounds: Vec<(TypeParamBound, Option<Token![+]>)>,
}

impl Parse for TypeParamBounds {
  fn parse(input: Stream) -> Option<Self> {
    let mut bounds = Vec::new();

    while let Some(parameter) = input.parse() {
      bounds.push((parameter, input.parse()));
    }

    if bounds.is_empty() {
      return None;
    }

    Some(TypeParamBounds { bounds })
  }
}

impl Collect for TypeParamBounds {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.bounds.collect(tree);
  }
}

///
/// - [TypeParamBound] :
///   [Lifetime] | [TraitBound]
///
/// [TypeParamBound]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) enum TypeParamBound {
  TraitBound(TraitBound),
  Lifetime(Lifetime),
}

impl Parse for TypeParamBound {
  fn parse(input: Stream) -> Option<Self> {
    if let Some(bound) = input.parse() {
      return Some(TypeParamBound::TraitBound(bound));
    }

    Some(TypeParamBound::Lifetime(input.parse()?))
  }
}

impl Collect for TypeParamBound {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    match self {
      Self::TraitBound(bound) => bound.collect(tree),
      Self::Lifetime(lifetime) => lifetime.collect(tree),
    }
  }
}

///
/// - [TraitBound] :
///          `?`? [ForLifetimes]? [TypePath]
///    | `(` `?`? [ForLifetimes]? [TypePath] `)`
///
/// [TraitBound]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) enum TraitBound {
  Path((Option<Token![?]>, TypePath)),
  Group(Group![()]), // TODO: Parse underlying group.
}

impl Parse for TraitBound {
  fn parse(input: Stream) -> Option<Self> {
    if let Some(group) = input.parse() {
      return Some(TraitBound::Group(group));
    }

    let ahead = input.fork();
    let question = ahead.parse();
    let _ = ahead.parse::<ForLifetimes>(); // TODO
    let path = ahead.parse()?; // Early return
    input.merge(ahead); // Move the cursor.
    Some(TraitBound::Path((question, path)))
  }
}

impl Collect for TraitBound {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    match self {
      Self::Path(path) => path.collect(tree),
      Self::Group(group) => group.collect(tree),
    }
  }
}

///
/// - [ForLifetimes] :
///   `for` [GenericParams]
///
/// [GenericParams]: crate::parser::generics::GenericParams
/// [ForLifetimes]: https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds
///
#[derive(Debug)]
pub(crate) struct ForLifetimes {
  _todo: (), // TODO
}

impl Parse for ForLifetimes {
  fn parse(input: Stream) -> Option<Self> {
    if input.parse::<Token![for]>().is_some() {
      panic!("Higher-ranked trait bounds are not supported yet.");
    }

    None
  }
}

impl Collect for ForLifetimes {
  fn collect(&self, _tree: &mut Vec<TokenTree>) {
    todo!()
  }
}

///
/// - [TypePath] :
///   `::`? [TypePathSegment] (`::` [TypePathSegment])*
///
/// [TypePath]: https://doc.rust-lang.org/reference/paths.html#paths-in-types
///
#[derive(Debug)]
pub(crate) struct TypePath {
  // TODO: Implement TypePathSegment instead of directly PathIdentSegment.
  pub(super) segments: Vec<(Option<(Token![:], Token![:])>, PathIdentSegment)>,
}

impl Parse for TypePath {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork(); // All or nothing.
    let mut segments = Vec::new();

    loop {
      let far_ahead = ahead.fork();
      // Path separators are mandatory, but we don't look at all of them
      // because we assume the stream is syntactically valid.
      let colon1 = far_ahead.parse::<Token![:]>();
      let colon2 = far_ahead.parse::<Token![:]>();
      if colon1.is_some() && colon2.is_none() {
        break;
      }

      if let Some(segment) = far_ahead.parse() {
        ahead.merge(far_ahead); // "::segment" is valid
        let separator = colon1.map(|colon1| (colon1, colon2.unwrap()));
        segments.push((separator, segment));
      } else {
        break; // Invalid segment.
      }
    }

    if segments.is_empty() {
      return None;
    }

    input.merge(ahead); // Move cursor.
    Some(TypePath { segments })
  }
}

impl Collect for TypePath {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.segments.collect(tree);
  }
}

///
/// - [TypePathSegment] :
///    [PathIdentSegment] (`::`? ([GenericArgs] | [TypePathFn]))?
///
/// - [TypePathFn] :
///   `(` [TypePathFnInputs]? `)` (`->` [TypeNoBounds])?
///
/// - [TypePathFnInputs] :
///   [Type] (`,` [Type])* `,`?
///
/// [TypePathSegment]: https://doc.rust-lang.org/reference/paths.html#paths-in-types
/// [GenericArgs]: https://doc.rust-lang.org/reference/paths.html#paths-in-expressions
/// [TypePathFn]: https://doc.rust-lang.org/reference/paths.html#paths-in-types
/// [TypePathFnInputs]: https://doc.rust-lang.org/reference/paths.html#paths-in-types
/// [TypeNoBounds]: https://doc.rust-lang.org/reference/types.html#type-expressions
/// [Type]: https://doc.rust-lang.org/reference/types.html#type-expressions
///
#[derive(Debug)]
#[allow(unused)]
pub(crate) struct TypePathSegment {
  _todo: (), // TODO
}

impl Parse for TypePathSegment {
  fn parse(_input: Stream) -> Option<Self> {
    todo!()
  }
}

impl Collect for TypePathSegment {
  fn collect(&self, _tree: &mut Vec<TokenTree>) {
    todo!()
  }
}

///
/// - [PathIdentSegment] :
///   [Identifier] | `super` | `self` | `Self` | `crate` | `$crate`
///
/// [PathIdentSegment]: https://doc.rust-lang.org/reference/paths.html#paths-in-expressions
///
#[derive(Debug)]
pub(crate) enum PathIdentSegment {
  Identifier(Identifier),
  Crate((Token![$], Token![crate])),
}

impl Parse for PathIdentSegment {
  fn parse(input: Stream) -> Option<Self> {
    if let Some(identifier) = input.parse() {
      return Some(PathIdentSegment::Identifier(identifier));
    }

    let ahead = input.fork();
    match (ahead.parse(), ahead.parse()) {
      (Some(dollar), Some(keyword)) => {
        input.merge(ahead); // Move the cursor.
        Some(PathIdentSegment::Crate((dollar, keyword)))
      }
      _ => None,
    }
  }
}

impl Collect for PathIdentSegment {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    match self {
      Self::Identifier(identifier) => identifier.collect(tree),
      Self::Crate(segment) => segment.collect(tree),
    }
  }
}
