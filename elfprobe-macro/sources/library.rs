mod cursor;
mod token;

use proc_macro::TokenStream;
use token::TokenCursor;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  let mut cursor = TokenCursor::from(input);
  println!("prev {:?}", cursor.prev());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("prev {:?}", cursor.prev());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("prev {:?}", cursor.prev());
  println!("prev {:?}", cursor.prev());
  println!("prev {:?}", cursor.prev());
  println!("prev {:?}", cursor.prev());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("prev {:?}", cursor.prev());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  println!("next {:?}", cursor.next());
  TokenStream::new()
}
