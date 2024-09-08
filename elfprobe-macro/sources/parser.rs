use proc_macro::TokenTree;

use crate::cursor::Cursor;
use crate::entry::Identifier;
use crate::entry::Literal;
use crate::token::Group;
use crate::token::Token;

pub(crate) type Stream<'buffer> = &'buffer Cursor<'buffer>;

pub(crate) trait Parse: Sized {
  // Parses and moves the cursor.
  fn parse(stream: Stream) -> Option<Self>;
}

pub(crate) trait Peek {
  // Checks required match, does not move the cursor.
  fn peek(stream: Stream) -> bool;
}

pub(crate) trait Collect {
  fn collect(&self, tree: &mut Vec<TokenTree>);
}

impl<Type: Collect> Collect for Option<Type> {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    if let Some(value) = self {
      value.collect(tree);
    }
  }
}

impl<Type1: Collect, Type2: Collect> Collect for (Type1, Type2) {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    self.0.collect(tree);
    self.1.collect(tree);
  }
}

impl<Type: Collect> Collect for Vec<Type> {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    for value in self.iter() {
      value.collect(tree);
    }
  }
}

///
/// - [OuterAttribute] :
///   `#` `[` [Attr] `]`
///
/// [OuterAttribute]: https://doc.rust-lang.org/reference/attributes.html
/// [Attr]: https://doc.rust-lang.org/reference/attributes.html
///
#[derive(Debug)]
pub(crate) struct OuterAttribute {
  hash_token: Token![#],
  attr_group: Group![[]], // TODO: Parse underlying group.
}

impl Parse for OuterAttribute {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork();

    // Early return without moving the cursor.
    let value = Some(OuterAttribute {
      hash_token: ahead.parse()?,
      attr_group: ahead.parse()?,
    });

    input.merge(ahead);
    value
  }
}

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

///
/// - [GenericParams] :
///   `<` `>` | `<` ([GenericParam] `,`)* [GenericParam] `,`? `>`
///
/// [GenericParams]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
///
#[derive(Debug)]
pub(crate) struct GenericParams {
  opening_angle_bracket: Token![<],
  closing_angle_bracket: Token![>],
  parameters: Vec<(GenericParam, Option<Token![,]>)>,
}

impl Parse for GenericParams {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork(); // All or nothing.
    let opening = ahead.parse::<Token![<]>()?;

    let mut parameters = Vec::new();
    while let Some(parameter) = ahead.parse() {
      // Commas are mandatory, but we don't look at them
      // because we assume the stream is syntactically valid.
      parameters.push((parameter, ahead.parse()));
    }

    let closing = ahead.parse::<Token![>]>()?;
    input.merge(ahead); // Move the cursor.
    Some(GenericParams {
      opening_angle_bracket: opening,
      closing_angle_bracket: closing,
      parameters,
    })
  }
}

impl GenericParams {
  pub(crate) fn collect_impl(&self) -> Vec<TokenTree> {
    let mut tree = Vec::<TokenTree>::new();
    self.opening_angle_bracket.collect(&mut tree);
    self.parameters.collect(&mut tree);
    self.closing_angle_bracket.collect(&mut tree);
    tree
  }

  pub(crate) fn collect_types(&self) -> Vec<TokenTree> {
    let mut tree = Vec::<TokenTree>::new();
    self.opening_angle_bracket.collect(&mut tree);
    for (parameter, comma) in self.parameters.iter() {
      match &parameter {
        GenericParam::Const(_const) => todo!(),
        GenericParam::Lifetime(LifetimeParam { lifetime, ..}) => {
          lifetime.collect(&mut tree);
        }
        GenericParam::Type(TypeParam { identifier, .. }) => {
          identifier.collect(&mut tree);
        }
      }
      comma.collect(&mut tree);
    }
    self.closing_angle_bracket.collect(&mut tree);
    tree
  }
}

///
/// - [GenericParam] :
///   [OuterAttribute]* ( [LifetimeParam] | [TypeParam] | [ConstParam] )
///
/// [GenericParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
///
#[derive(Debug)]
pub(crate) enum GenericParam {
  Const(ConstParam),
  Lifetime(LifetimeParam),
  Type(TypeParam),
}

impl Parse for GenericParam {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork();

    // TODO: Store outer attributes.
    if ahead.parse::<OuterAttribute>().is_some() {
      todo!("Outer attributes are not supported for generic parameters yet.");
    }

    if let Some(lifetime) = ahead.parse() {
      input.merge(ahead); // Move the cursor
      return Some(GenericParam::Lifetime(lifetime));
    }

    if let Some(parameter) = ahead.parse() {
      input.merge(ahead); // Move the cursor.
      return Some(GenericParam::Type(parameter));
    }

    if let Some(parameter) = ahead.parse() {
      input.merge(ahead); // Move the cursor.
      return Some(GenericParam::Const(parameter));
    }

    None
  }
}

impl Collect for GenericParam {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    match self {
      Self::Const(parameter) => parameter.collect(tree),
      Self::Lifetime(parameter) => parameter.collect(tree),
      Self::Type(parameter) => parameter.collect(tree),
    }
  }
}

///
/// - [ConstParam] :
///   `const` [Identifier] `:` [Type] ( `=` [Block] | [Identifier] | -? [Literal] )?
///
/// [ConstParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
/// [Block]: https://doc.rust-lang.org/reference/expressions/block-expr.html
/// [Type]: https://doc.rust-lang.org/reference/types.html#type-expressions
///
#[derive(Debug)]
pub(crate) struct ConstParam {
  todo: (), // TODO
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
  fn collect(&self, tree: &mut Vec<TokenTree>) {
    todo!()
  }
}

///
/// - [LifetimeParam]: \
///   [LifetimeOrLabel] ( `:` [LifetimeBounds] )?
///
/// [LifetimeParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
///
#[derive(Debug)]
pub(crate) struct LifetimeParam {
  lifetime: LifetimeOrLabel,
  bounds: Option<(Token![:], LifetimeBounds)>,
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
  quote: Token![simple_quote],
  lifetime: Identifier,
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
  lifetimes: Vec<(Lifetime, Option<Token![+]>)>,
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

///
/// - [TypeParam] :
///   [Identifier] ( `:` [TypeParamBounds]? )? ( `=` [Type] )?
///
/// [TypeParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
/// [Type]: https://doc.rust-lang.org/reference/types.html#type-expressions
///
#[derive(Debug)]
pub(crate) struct TypeParam {
  identifier: Identifier,
  bounds: Option<(Token![:], Option<TypeParamBounds>)>,
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
  bounds: Vec<(TypeParamBound, Option<Token![+]>)>,
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
/// [ForLifetimes]: https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds
///
#[derive(Debug)]
pub(crate) struct ForLifetimes {
  // TODO
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
  fn collect(&self, tree: &mut Vec<TokenTree>) {
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
  segments: Vec<(Option<(Token![:], Token![:])>, PathIdentSegment)>,
}

impl Parse for TypePath {
  fn parse(input: Stream) -> Option<Self> {
    let ahead = input.fork(); // All or nothing.
    let mut segments = Vec::new();

    loop {
      let goldorak = ahead.fork();
      // Path separators are mandatory, but we don't look at all of them
      // because we assume the stream is syntactically valid.
      let colon1 = goldorak.parse::<Token![:]>();
      let colon2 = goldorak.parse::<Token![:]>();
      if colon1.is_some() && colon2.is_none() {
        break;
      }

      if let Some(segment) = goldorak.parse() {
        ahead.merge(goldorak); // "::segment" is valid
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
pub(crate) struct TypePathSegment {
  // TODO
}

impl Parse for TypePathSegment {
  fn parse(input: Stream) -> Option<Self> {
    todo!()
  }
}

impl Collect for TypePathSegment {
  fn collect(&self, tree: &mut Vec<TokenTree>) {
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
