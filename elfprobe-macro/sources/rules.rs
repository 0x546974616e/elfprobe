use proc_macro::TokenTree;

use crate::entry::Identifier;
use crate::parser::parser;
use crate::parser::Collect;
use crate::parser::Union;
use crate::token::*;

// ╦═╗┬ ┬┬  ┌─┐┌─┐
// ╠╦╝│ ││  ├┤ └─┐
// ╩╚═└─┘┴─┘└─┘└─┘

// https://doc.rust-lang.org/reference/items/structs.html#structs
parser!(StructType = [OuterAttribute*] [Visibility?] (StructStruct | TupleStruct));
parser!(StructStruct = Struct Identifier [GenericParams?] [WhereClause?] (Brace | SemiColon));
parser!(TupleStruct = Struct Identifier [GenericParams?] Parenthesis [WhereClause?] SemiColon);

// https://doc.rust-lang.org/reference/attributes.html
parser!(OuterAttribute = Hash Bracket );

// https://doc.rust-lang.org/reference/visibility-and-privacy.html
parser!(Visibility = Pub[Parenthesis?]);

// https://doc.rust-lang.org/reference/items/generics.html#generic-parameters
parser!(GenericParams = Lt [(GenericParam [Comma?])*] Gt);
parser!(GenericParam = [OuterAttribute*] (LifetimeParam | TypeParam | ConstParam));
parser!(LifetimeParam = Lifetime [(Colon LifetimeBounds)?]);
parser!(TypeParam = Identifier [(Colon TypeParamBounds)?]); // TODO: "= Type"
parser!(ConstParam = Const Identifier Colon Identifier); // TODO: ": Type (= Block | Identifier | Literal)?"

// https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels
parser!(Lifetime = Quote Identifier); // LifetimeOrLabel

// https://doc.rust-lang.org/reference/trait-bounds.html#trait-and-lifetime-bounds
parser!(TypeParamBounds = [(TypeParamBound [Plus?])*]);
parser!(TypeParamBound = Lifetime | TraitBound);
parser!(TraitBound = Parenthesis | TypePath); // TODO: "? ForLifetimes"
parser!(LifetimeBounds = [(Lifetime [Plus?])*]);

// https://doc.rust-lang.org/reference/paths.html#paths-in-types
parser!(DoubleColon = Colon Colon);
parser!(TypePath = [([DoubleColon?] Identifier)+]); // TODO: "TypePathSegment" instead of "Identifier"
parser!(TypePathSegment = PathIdentSegment); // TODO: ":: (GenericArgs | TypePathFn)"

// https://doc.rust-lang.org/reference/paths.html#paths-in-expressions
parser!(PathIdentSegment = Identifier); // TODO: "$crate"

// https://doc.rust-lang.org/reference/items/generics.html#where-clauses
parser!(WhereClause = Where [(WhereClauseItem [Comma?])+]);
parser!(WhereClauseItem = LifetimeWhereClauseItem | TypeBoundWhereClauseItem);
parser!(LifetimeWhereClauseItem = Lifetime Colon LifetimeBounds);
parser!(TypeBoundWhereClauseItem = Identifier Colon TypeParamBounds); // TODO: "ForLifetimes Type:"

// ╔═╗┬ ┬┌─┐┌┬┐┌─┐┌┬┐
// ║  │ │└─┐ │ │ ││││
// ╚═╝└─┘└─┘ ┴ └─┘┴ ┴

impl StructType {
  /// Returns the name of the structure.
  pub(crate) fn name(&self) -> &Identifier {
    match &self.tree.2 {
      Union::A(struct_struct) => &struct_struct.tree.1,
      Union::B(tuple_struct) => &tuple_struct.tree.1,
      _ => unreachable!(),
    }
  }

  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(crate) fn collect_impl(&self) -> Vec<TokenTree> {
    let mut tree = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_impl_into(&mut tree),
      Union::B(tuple_struct) => tuple_struct.collect_impl_into(&mut tree),
      _ => unreachable!(),
    }
    tree
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(crate) fn collect_types(&self) -> Vec<TokenTree> {
    let mut tree = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_types_into(&mut tree),
      Union::B(tuple_struct) => tuple_struct.collect_types_into(&mut tree),
      _ => unreachable!(),
    }
    tree
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(crate) fn collect_where_clause(&self) -> Vec<TokenTree> {
    let mut tree = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_where_clause_into(&mut tree),
      Union::B(tuple_struct) => tuple_struct.collect_where_clause_into(&mut tree),
      _ => unreachable!(),
    }
    tree
  }
}

impl StructStruct {
  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(crate) fn collect_impl_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_into(tree);
    }
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(crate) fn collect_types_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_types_into(tree);
    }
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(crate) fn collect_where_clause_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(r#where) = &self.tree.3 {
      r#where.collect_into(tree);
    }
  }
}

impl TupleStruct {
  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(crate) fn collect_impl_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_into(tree);
    }
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(crate) fn collect_types_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_types_into(tree);
    }
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(crate) fn collect_where_clause_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(r#where) = &self.tree.4 {
      r#where.collect_into(tree);
    }
  }
}

impl GenericParams {
  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(crate) fn collect_types_into(&self, tree: &mut Vec<TokenTree>) {
    // Collect the opening angle bracket.
    self.tree.0.collect_into(tree);

    // Collect all generic identifiers only.
    for (generic, comma) in self.tree.1.iter() {
      match &generic.tree.1 {
        Union::A(lifetime) => lifetime.tree.0.collect_into(tree),
        Union::B(parameter) => parameter.tree.0.collect_into(tree),
        Union::C(constant) => constant.tree.0.collect_into(tree),
        _ => (),
      }

      // Collect the generics separator (comma).
      comma.collect_into(tree);
    }

    // Collect the closing angle bracket.
    self.tree.2.collect_into(tree);
  }
}
