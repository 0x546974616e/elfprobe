use proc_macro::TokenStream;

use crate::buffer::Buffer;
use crate::entry::Group;
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
/// - [Struct] :
///   [OuterAttribute]* [Visibility]? ( [StructStruct] | [TupleStruct] )
///
/// - [StructStruct] :
///   `struct` [Identifier] [GenericParams]? [WhereClause]? ( `{` [StructFields]? `}` | `;` )
///
/// - [TupleStruct] :
///   `struct` [Identifier] [GenericParams]? `(` [TupleFields]? `)` [WhereClause]? `;`
///
/// [Struct]: https://doc.rust-lang.org/reference/items/structs.html#structs
/// [StructStruct]: https://doc.rust-lang.org/reference/items/structs.html#structs
/// [TupleStruct]: https://doc.rust-lang.org/reference/items/structs.html#structs
/// [StructFields]: https://doc.rust-lang.org/reference/items/structs.html#structs
/// [TupleFields]: https://doc.rust-lang.org/reference/items/structs.html#structs
/// [WhereClause]: https://doc.rust-lang.org/reference/items/generics.html#where-clauses
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

  // 6. Where clause (structure) (optional)
  if cursor.parse::<Token![where]>().is_some() {
    panic!("Where clause are not supported yet.")
  }

  // 7. Structure or tuple fiels (optional)
  cursor.parse::<Group>();

  // 8. Where clause (tuple) (optional)
  if cursor.parse::<Token![where]>().is_some() {
    panic!("Where clause are not supported yet.")
  }

  // 9. Semicolon (mandatory)
  cursor
    .parse::<Token![;]>()
    .expect("Expected a semicolon (`;`) after `struct` declaration.");

  if !cursor.is_end() {
    panic!("Expected the end of the token stream.")
  }

  eprintln!("{:?}", cursor.entry());
}
