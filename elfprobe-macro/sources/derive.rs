use proc_macro::TokenStream;

use crate::buffer::Buffer;
use crate::entry::Identifier;
use crate::parser::GenericParams;
use crate::parser::OuterAttribute;
use crate::parser::Visibility;
use crate::token::Token;

// TODO: https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html (when stable)

// pub struct #GENERICS Dada() where #WHERE;
// impl #GENERICS Pod for Dada #TYPES where #WHERE {}

// (impl_generics, type_generics, where_clause)
// impl #impl_generics crate::pod::Pod for #name #type_generics #where_clause {}

///
/// - [Struct] : \
///   [OuterAttribute]* [Visibility]? ( [StructStruct] | [TupleStruct] )
///
/// - [StructStruct] : \
///   `struct` [IDENTIFIER] [GenericParams]? [WhereClause]? ( `{` [StructFields]? `}` | `;` )
///
/// - [TupleStruct] : \
///   `struct` [IDENTIFIER] [GenericParams]? `(` [TupleFields]? `)` [WhereClause]? `;`
///
/// [Struct]: https://doc.rust-lang.org/reference/items/structs.html
/// [StructStruct]: https://doc.rust-lang.org/reference/items/structs.html
/// [TupleStruct]: https://doc.rust-lang.org/reference/items/structs.html
///
pub(crate) fn derive(stream: TokenStream) {
  let buffer = Buffer::from(stream);
  let cursor = buffer.cursor();
  // eprintln!("{:#?}", buffer);

  // 1. Outer attributes (optional)
  while cursor.parse::<OuterAttribute>().is_some() {}

  // 2. Visibility (optional)
  cursor.parse::<Visibility>();

  // 3. "struct" keywork (required)
  cursor
    .parse::<Token![struct]>()
    .expect("Expected `struct` keyword (enumerations are not supported yet).");

  // 4. Structure identifier (required)
  let Some(name) = cursor.parse::<Identifier>() else {
    panic!("Expected structure identifier.")
  };

  // 5. Generic parameters (optional)
  let generics = cursor.parse::<GenericParams>();
  eprintln!("{:#?}", generics);

  eprintln!("{:?}", cursor.entry());
}
