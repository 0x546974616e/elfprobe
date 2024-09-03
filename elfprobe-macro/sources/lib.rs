use proc_macro::TokenStream;
use quote::quote;
use syn;

// https://crates.io/crates/syn
// https://crates.io/crates/quote

// https://doc.rust-lang.org/stable/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://stackoverflow.com/questions/76705814/how-can-i-use-derive-macro-on-a-generic-struct

// David Tolnay
// "Oh it's by dtolnay, I feel much better."
// https://github.com/dtolnay

// https://internals.rust-lang.org/t/announcement-david-tolnay-joining-the-libs-team/5186
// https://dev.to/szabgab/github-sponsor-rust-developer-david-tolnay-53kc
// https://www.reddit.com/r/rust/comments/mify2o/david_tolnay_thank_you/

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_pod_derive(&ast)
}

fn impl_pod_derive(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
  let expanded = quote! {
    impl #impl_generics crate::pod::Pod for #name #type_generics #where_clause {}
  };
  expanded.into()
}

// https://developerlife.com/2022/03/30/rust-proc-macro/
// https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
// https://www.reddit.com/r/rust/comments/hq1aa3/a_reference_for_creating_proc_macros_without/

// https://crates.io/crates/hex-literal/0.4.1
// https://github.com/RustCrypto/utils/blob/master/hex-literal/src/lib.rs
// https://github.com/dtolnay/syn/blob/master/src/generics.rs

// https://doc.rust-lang.org/reference/procedural-macros.html
// https://github.com/landaire/rust-proc-macro-without-dependencies/blob/master/default_derive/src/lib.rs

// https://doc.rust-lang.org/reference/macros-by-example.html
// https://doc.rust-lang.org/reference/types/struct.html
// https://doc.rust-lang.org/reference/items/structs.html
// https://doc.rust-lang.org/reference/expressions/struct-expr.html
