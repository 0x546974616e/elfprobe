
# elfprobe-macro

Export all [procedural macros][proc_macro] for the entire project.

[proc_macro]: https://doc.rust-lang.org/reference/procedural-macros.html

## How to `#[derive(Pod)]`

Here two solutions to implement the `Pod` derive macro.

### Solution 1: Write a `TokenStream` parser by hand from scratch

See [`sources/`](./sources/) (still WIP).

So far, only a subset of Rust language is supported.

### Solution 2: Use `syn` and `quote` crates

> "Oh it's by dtolnay, I feel much better." (Twitter)

(Even from [David Tolnay][dtolnay], I do not want any non-standard external
library, especially for something that should be standard...)

[dtolnay]: https://github.com/dtolnay

1. `Cargo.toml`

`cargo new elfprobe-macro --lib --vcs none`

```toml
[package]
name = "elfprobe-macro"
version = "0.1.0"
edition = "2021"

[lib]
name = "elfprobe_macro"
path = "sources/library.rs"
proc-macro = true

[dependencies]
[dependencies.syn]
version = "2.0"
default-features = false
features = [
  "derive",
  "parsing",
  "printing",
  "proc-macro",
]
[dependencies.quote]
version = "1.0"
default-features = false
```

2. `sources/library.rs`

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

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
```

3. `tests/test_macro.rs`

```rust
#[test]
fn dada() {
  #[allow(unused)]
  use elfprobe_macro::Pod;
  use std::fmt::Display;

  #[allow(unused)]
  #[derive(Pod)]
  struct Dada<T: Default, U>(T, U)
    where U: Display;
}
```
