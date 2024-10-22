use proc_macro::TokenTree;

use crate::parser::Collect;
use crate::parser::Identifier;
use crate::parser::Union;

use crate::parser::rules::GenericParams;
use crate::parser::rules::StructStruct;
use crate::parser::rules::StructType;
use crate::parser::rules::TupleStruct;

impl StructType {
  /// Returns the name of the structure.
  pub(super) fn name(&self) -> &Identifier {
    match &self.tree.2 {
      Union::A(struct_struct) => &struct_struct.tree.1,
      Union::B(tuple_struct) => &tuple_struct.tree.1,
      _ => unreachable!(),
    }
  }

  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(super) fn collect_impl(&self) -> Vec<TokenTree> {
    let mut tokens = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_impl_into(&mut tokens),
      Union::B(tuple_struct) => tuple_struct.collect_impl_into(&mut tokens),
      _ => unreachable!(),
    }
    tokens
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(super) fn collect_types(&self) -> Vec<TokenTree> {
    let mut tokens = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_types_into(&mut tokens),
      Union::B(tuple_struct) => tuple_struct.collect_types_into(&mut tokens),
      _ => unreachable!(),
    }
    tokens
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(super) fn collect_where_clause(&self) -> Vec<TokenTree> {
    let mut tokens = Vec::new();
    match &self.tree.2 {
      Union::A(struct_struct) => struct_struct.collect_where_clause_into(&mut tokens),
      Union::B(tuple_struct) => tuple_struct.collect_where_clause_into(&mut tokens),
      _ => unreachable!(),
    }
    tokens
  }
}

impl StructStruct {
  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(super) fn collect_impl_into(&self, tokens: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_into(tokens);
    }
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(super) fn collect_types_into(&self, tokens: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_types_into(tokens);
    }
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(super) fn collect_where_clause_into(&self, tokens: &mut Vec<TokenTree>) {
    if let Some(r#where) = &self.tree.3 {
      r#where.collect_into(tokens);
    }
  }
}

impl TupleStruct {
  /// Returns all generics with their bounds,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>`.
  pub(super) fn collect_impl_into(&self, tokens: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_into(tokens);
    }
  }

  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(super) fn collect_types_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(generics) = &self.tree.2 {
      generics.collect_types_into(tree);
    }
  }

  /// Returns the structure where clause, e.g., `where A: Default, B: 'a + foo::Bar`
  pub(super) fn collect_where_clause_into(&self, tree: &mut Vec<TokenTree>) {
    if let Some(r#where) = &self.tree.4 {
      r#where.collect_into(tree);
    }
  }
}

impl GenericParams {
  /// Returns all generic identifiers only,
  /// e.g., `<'a, 'b: 'a + Default, A, B: Debug>` gives `<'a, 'b, A, b>`.
  pub(super) fn collect_types_into(&self, tree: &mut Vec<TokenTree>) {
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
