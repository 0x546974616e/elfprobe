use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::Ident;

pub fn is_hex(string: &str) -> bool {
  string.starts_with("0x") && string.chars().skip(2).all(|char| char.is_ascii_hexdigit())
}

pub fn is_bin(string: &str) -> bool {
  string.starts_with("0b") && string.chars().skip(2).all(|char| char == '0' || char == '1')
}

pub fn map_boolean(input: TokenStream, predicate: impl Fn(&str) -> bool) -> TokenStream {
  let mut output = TokenStream::new();

  const BOOLEANS: [&str; 2] = [ "false", "true" ];
  output.extend(input.into_iter().map(|token| match token {
    TokenTree::Literal(literal) => TokenTree::from(Ident::new(
      // predicate(literal.to_string().as_ref()).to_string().as_str(),
      BOOLEANS[predicate(literal.to_string().as_ref()) as usize],
      literal.span(),
    )),
    _ => token,
  }));

  output
}
