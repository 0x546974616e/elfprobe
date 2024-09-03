#[allow(unused)]
use std::mem::align_of;
use std::mem::size_of;

use crate::error::BytesError;

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
pub trait Pod: 'static + Copy + Sized {
  #[allow(clippy::needless_lifetimes)] // For readability.
  fn from_bytes<'data>(bytes: &'data [u8]) -> Result<&'data Self, BytesError> {
    if bytes.len() != size_of::<Self>() {
      return Err(BytesError::SizeOfMismatch {
        length: bytes.len(),
        size_of: size_of::<Self>(),
      });
    }

    let pointer = bytes.as_ptr();
    #[cfg(any(clippy, not(feature = "unaligned")))]
    if (pointer as usize) % align_of::<Self>() != 0 {
      return Err(BytesError::AlignOfMismatch {
        pointer: pointer as usize,
        align_of: align_of::<Self>(),
      });
    }

    // What about std::ptr::read*() methods?
    // What kind of security do they provide?
    // https://doc.rust-lang.org/std/ptr/fn.read.html
    // https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
    // https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html

    // From read_volatile()
    // Rust does not currently have a rigorously and formally defined memory
    // model, so the precise semantics of what “volatile” means here is subject
    // to change over time. That being said, the semantics will almost always
    // end up pretty similar to C11’s definition of volatile.

    // From INTERNATIONAL STANDARD - Programming languages - C11
    // https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
    // Page 122, §6.7.3, footnote 134
    // A **volatile** declaration may be used to describe an object
    // corresponding to a memory-mapped input/output port or an object accessed
    // by an asynchronously interrupting function. Actions on objects so
    // declared shall not be ‘‘optimized out’’ by an implementation or reordered
    // except as permitted by the rules for evaluating expressions.

    // From https://stackoverflow.com/a/29102709
    // This is one of the two situations where volatile is mandatory (and it
    // would be nice if compilers could know that).
    //
    // Any memory location which can change either without your code initiating
    // it (I.e. a memory mapped device register) or without your thread
    // initiating it (i.e. it is changed by another thread or by an interrupt
    // handler) absolutely must be declared as volatile to prevent the compiler
    // optimizing away memory-fetch operations.
    //
    // Your answer is incomplete, as it only focuses on the fetch aspect.
    // There's a complimentary requirement for store.

    Ok(unsafe { &*pointer.cast::<Self>() })
  }
}

#[allow(unused_macros)]
macro_rules! impl_pod {
  ($($bytes: literal),+, $($type: ident),+) => {
    $(impl Pod for [u8; $bytes] {})+
    $(impl Pod for $type {})+
  };
}

// Implement POD trait for primitive types in order to be used by POD aggregates.
impl_pod!(2, 4, 8, i8, u8, i16, u16, i32, u32, i64, u64);

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem::offset_of;

  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  struct Dada {
    a: u64,
    b: u32,
    c: u16,
    d: u8,
  }

  impl Pod for Dada {}

  impl Default for Dada {
    fn default() -> Self {
      // "Bypass" endianness.
      Self {
        a: 0x04_04_04_04_04_04_04_04_u64,
        b: 0x03_03_03_03_u32,
        c: 0x02_02_u16,
        d: 0x01_u8,
      }
    }
  }

  #[test]
  fn from_bytes_ok() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);
    assert_eq!(offset_of!(Dada, a), 0);
    assert_eq!(offset_of!(Dada, b), 8);
    assert_eq!(offset_of!(Dada, c), 12);
    assert_eq!(offset_of!(Dada, d), 14);

    let bytes: [u8; 16] = [
      // Byte order has no consequence in this way.
      0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, // a
      0x03, 0x03, 0x03, 0x03, // b
      0x02, 0x02, // c
      0x01, // d
      0x00, //
    ];

    let dada = Dada::from_bytes(&bytes);
    assert_eq!(Ok(&Dada::default()), dada);
  }

  #[test]
  fn from_bytes_size_of_error() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);

    assert_eq!(
      Dada::from_bytes(&[1, 2, 3]),
      Err(BytesError::SizeOfMismatch {
        length: 3,
        size_of: 16,
      }),
    )
  }

  #[test]
  #[cfg(any(clippy, not(feature = "unaligned")))]
  fn from_bytes_align_of_error() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);

    let bytes: &[u8; 1 + 16] = &[
      0x00, // To make sure it is unaligned.
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];

    let slice = &bytes[1..];
    assert_eq!(
      Dada::from_bytes(slice),
      Err(BytesError::AlignOfMismatch {
        pointer: slice.as_ptr() as usize,
        align_of: 8,
      }),
    )
  }
}
