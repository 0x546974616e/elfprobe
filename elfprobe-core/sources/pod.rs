///
/// TLDR: A POD type is a bag of bits with no magic.
///
/// A POD (Plain Old Data) type is all primitive types (`u8`, `i32`, `i64`...)
/// and all aggregations of POD types (`struct`, `union`...). A POD structure
/// contains only POD types as members and does not have any constructors,
/// destructors and virtual members functions.
///
/// The following trait bounds are here to enforce [the idea of POD type in
/// Rust][rust_pod]:
///
/// - [`'static`][static] as a trait bound means that the type does not contain
///   any internal non-static references (`&T` and `&mut T`). Type that are
///   `'static` have therefore no lifetime restrictions and will be basically
///   ignored by the borrow checker.
///
/// - [`Copy`] trait allows values to be duplicated simply by copying its bits
///   (no move semantics). `Copy` trait is then implemented by types that do not
///   have complex memory management with for example heap allocation (pointers)
///   or shared mutable references (`&mut T`, note that `&T` is `Copy` though).
///
/// - [`Sized`] trait requires that the type has a size known at compile time
///   and can thus be stored on the stack.
///
/// - [`Send`] and [`Sync`] require that the type can be sent to other threads
///   and can shared via immutable reference (`&T`) across threads. These traits
///   are not implemented if the type contains some kind of magic (interior
///   mutability, references without a lifetime...).
///
/// [rust_pod]: https://stackoverflow.com/questions/45634083/is-there-a-concept-of-pod-types-in-rust
/// [static]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
///
#[allow(unused)]
// TODO: Add Send + Sync
pub trait Pod: 'static + Copy + Sized {}

#[allow(unused_macros)]
macro_rules! impl_pod {
  ($($type: ident),+) => {
    $(impl Pod for $type {})+
  };
}

// Implement POD trait for primitive types in order to be used by POD aggregates.
impl_pod!(i8, u8, i16, u16, i32, u32, i64, u64);
