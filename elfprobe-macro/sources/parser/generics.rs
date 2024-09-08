use proc_macro::TokenTree;

use crate::token::Token;

use super::attributes::OuterAttribute;
use super::constants::ConstParam;
use super::lifetimes::LifetimeParam;
use super::types::TypeParam;

use super::Collect;
use super::Parse;
use super::Stream;

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
        GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
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
