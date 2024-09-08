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

///
/// - [OuterAttribute] : \
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
/// - [Visibility] : \
///     `pub` \
///   | `pub` `(` `crate` `)` \
///   | `pub` `(` `self` `)` \
///   | `pub` `(` `super` `)` \
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
/// - [GenericParams] : \
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

///
/// - [GenericParam] : \
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

    if let Some(lifetime) = ahead.parse::<LifetimeParam>() {
      input.merge(ahead); // Move the cursor (I forgot this one).
      return Some(GenericParam::Lifetime(lifetime));
    }

    if let Some(parameter) = ahead.parse::<TypeParam>() {
      input.merge(ahead); // Move the cursor.
      return Some(GenericParam::Type(parameter));
    }

    if let Some(parameter) = ahead.parse::<ConstParam>() {
      input.merge(ahead); // Move the cursor.
      return Some(GenericParam::Const(parameter));
    }

    None
  }
}

///
/// - [ConstParam] : \
///   `const` [Identifier] `:` [Type] ( `=` [Block] | [Identifier] | -? [Literal] )?
///
/// [ConstParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
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

///
/// - [LifetimeOrLabel] : \
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

    // Early return without moving the cursor.
    let value = Some(LifetimeOrLabel {
      quote: ahead.parse()?,
      lifetime: ahead.parse()?,
    });

    input.merge(ahead);
    value
  }
}

///
/// - [LifetimeBounds] : \
///   ( [Lifetime] + )* [Lifetime]?
///
/// - [Lifetime] : \
///   [LifetimeOrLabel] | `'static` | `'_`
///
/// [LifetimeBounds]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
/// [Lifetime]: https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
///
#[derive(Debug)]
pub(crate) struct LifetimeBounds {
  lifetimes: Vec<(LifetimeOrLabel, Option<Token![+]>)>, // TODO: '_
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

///
/// - [TypeParam]: \
///   [Identifier] ( `:` [TypeParamBounds]? )? ( `=` [Type] )?
///
/// [TypeParam]: https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
///
#[derive(Debug)]
pub(crate) struct TypeParam {
  identifier: Identifier,
}

impl Parse for TypeParam {
  fn parse(input: Stream) -> Option<Self> {
    Some(TypeParam {
      identifier: input.parse::<Identifier>()?,
    })
  }
}
