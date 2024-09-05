mod cursor;

use proc_macro::TokenStream;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  println!("{:#?}", input);
  TokenStream::new()
}
