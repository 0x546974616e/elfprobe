use std::str::FromStr;

use proc_macro::Delimiter;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

mod buffer;
mod cursor;
mod entry;
mod parser;
mod rules;
mod token;

use buffer::Buffer;
use entry::Group;
use entry::Identifier;
use parser::Parse;
use rules::StructType;

///
/// ```txt
/// pub struct #GENERICS #IDENTIFIER {...} where #WHERE;
/// impl #IMPL_GENERICS #TRAIT for #IDENTIFIER #TYPE_GENERICS where #WHERE {}
/// ```
///
pub(crate) fn derive(stream: TokenStream, r#trait: &str) -> TokenStream {
  let buffer = Buffer::from(stream);
  let cursor = buffer.cursor();
  // eprintln!("{:#?}", buffer);

  // ╔═╗┌─┐┬─┐┌─┐┌─┐
  // ╠═╝├─┤├┬┘└─┐├┤
  // ╩  ┴ ┴┴└─└─┘└─┘

  // 1. Parse the given structure.
  let r#struct = StructType::parse(&cursor);

  // 2. A structure should have been found.
  if r#struct.is_none() {
    panic!("Could not parse the given structure (enumerations are not supported yet).");
  }

  // 3. The end of the stream should be reached.
  if !cursor.is_end() {
    panic!("Expected the end of the token stream.")
  }

  // ╔═╗┌─┐┌┐┌┌─┐┬─┐┌─┐┌┬┐┌─┐
  // ║ ╦├┤ │││├┤ ├┬┘├─┤ │ ├┤
  // ╚═╝└─┘┘└┘└─┘┴└─┴ ┴ ┴ └─┘

  // A. Build the TokenStream.
  let mut derive = TokenStream::new();

  // B. "impl" keyword
  derive.extend([TokenTree::from(Identifier::new("impl", Span::call_site()))]);

  // C. Structure generics
  if let Some(r#struct) = &r#struct {
    derive.extend(r#struct.collect_impl());
  }

  // D. Trait to derive
  derive.extend(TokenStream::from_str(r#trait));

  // E. "for" keyword
  derive.extend([TokenTree::from(Identifier::new("for", Span::call_site()))]);

  // F. Structure name.
  if let Some(r#struct) = &r#struct {
    derive.extend([TokenTree::from(r#struct.name().clone())]);
  }

  // G. Generic types.
  if let Some(r#struct) = &r#struct {
    derive.extend(r#struct.collect_types());
  }

  // H. Where clause
  if let Some(r#struct) = &r#struct {
    derive.extend(r#struct.collect_where_clause());
  }

  // I. Brace group
  derive.extend([TokenTree::from(Group::new(Delimiter::Brace, TokenStream::new()))]);

  derive
}
