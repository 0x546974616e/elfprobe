use std::str::FromStr;

use proc_macro::Delimiter;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

use crate::buffer::Buffer;
use crate::entry::Group;
use crate::entry::Identifier;
use crate::parser::attributes::OuterAttribute;
use crate::parser::generics::GenericParams;
use crate::parser::visibilities::Visibility;
use crate::token::Token;

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
pub(crate) fn derive(stream: TokenStream, traitt: &str) -> TokenStream {
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
  // eprintln!("{:#?}", generics);

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

  // 9. Semicolon (optional)
  cursor.parse::<Token![;]>();

  if !cursor.is_end() {
    panic!("Expected the end of the token stream.")
  }

  // 10. Build the TokenStream.

  //// pub struct #GENERICS Dada() where #WHERE;
  //// impl #IMPL_GENERICS Pod for Dada #TYPE_GENERICS where #WHERE {}
  let mut derive = TokenStream::new();

  // 11. "impl" keyworkd
  derive.extend([TokenTree::from(Identifier::new("impl", Span::call_site()))]);

  // 12. Structure generics
  if let Some(generics) = &generics {
    derive.extend(generics.collect_impl());
  }

  // 13. Trait to derive
  derive.extend(TokenStream::from_str(traitt));

  // 14. "for" keyword
  derive.extend([TokenTree::from(Identifier::new("for", Span::call_site()))]);

  // 15. Structure name.
  derive.extend([TokenTree::from(name)]);

  // 15. Generic types.
  if let Some(generics) = &generics {
    derive.extend(generics.collect_types());
  }

  // 16. Brace group
  derive.extend([TokenTree::from(Group::new(Delimiter::Brace, TokenStream::new()))]);

  derive
}
